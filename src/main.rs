use std::ops::Not;
use std::process::Command;

use kittylitters::collect_windows::collect_windows;
use kittylitters::first_value::get_first_values;
use kittylitters::read_yml::read_session_yml;
use kittylitters::set_logic::{Operation, Solver};
use kittylitters::windows::Window;

fn main() -> color_eyre::Result<()> {
    let current_windows = collect_windows().unwrap();

    let home_dir = std::env::var("HOME").expect("HOME environment variable should be set");
    let config_path = format!("{}/.config/kitty/session.yml", home_dir);
    let desired_windows = read_session_yml(&config_path)
        .expect("Expect a valid yml file at ~/.config/kitty/session.yml");

    let current_state_first_windows = get_first_values(&current_windows);
    let desired_state_first_windows = get_first_values(&desired_windows);

    let current_tabs: Vec<Window> = current_state_first_windows
        .iter()
        .map(|(tab_title, window_title)| Window::new(format!("{0}/{1}", tab_title, window_title)))
        .collect();

    let desired_tabs: Vec<Window> = desired_state_first_windows
        .iter()
        .map(|(tab_title, window)| {
            if let Some(window_title) = current_state_first_windows.get(tab_title) {
                Window::new(format!("{0}/{1}", tab_title, window_title))
            } else {
                let window_title = window
                    .get("title")
                    .and_then(|v| v.as_ref())
                    .expect("window_dict should have title");
                let cwd = window.get("cwd").and_then(|opt| opt.as_ref());
                let cmd = window.get("cmd").and_then(|opt| opt.as_ref());
                Window::new(format!("{0}/{1}", tab_title, window_title))
                    .with_cwd(cwd.map(|s| s.to_owned()))
                    .with_cmd(cmd.map(|s| s.to_owned()))
            }
        })
        .collect();

    let tab_operations = Solver::derive_operations(&current_tabs, &desired_tabs);

    for operation in tab_operations {
        match operation {
            Operation::GoTo(window) => {
                if Command::new("kitten")
                    .args([
                        "@",
                        "focus-window",
                        "--match",
                        &format!("title:^{}$", window.title),
                    ])
                    .status()
                    .is_ok_and(|x| x.success())
                    .not()
                {
                    color_eyre::eyre::bail!("Could not GoTo window `{}`", window.title);
                }
            }
            Operation::Close => {
                if Command::new("kitten")
                    .args(["@", "action", "close_tab"])
                    .status()
                    .is_ok_and(|x| x.success())
                    .not()
                {
                    color_eyre::eyre::bail!("Could not close tab"); // TODO: window name
                }
            }
            Operation::MoveForward(count) => {
                for _ in 0..count {
                    if Command::new("kitten")
                        .args(["@", "action", "move_tab_forward"])
                        .status()
                        .is_ok_and(|x| x.success())
                        .not()
                    {
                        color_eyre::eyre::bail!("Could not move tab forward"); // TODO: window name
                    }
                }
            }
            Operation::New(window) => {
                let mut args = vec!["@", "launch", "--type", "tab", "--title", &window.title];
                if let Some(cwd) = &window.cwd {
                    args.push("--cwd");
                    args.push(cwd);
                }
                if let Some(cmd) = &window.cmd {
                    args.push(cmd);
                }
                if Command::new("kitten")
                    .args(args)
                    .status()
                    .is_ok_and(|x| x.success())
                    .not()
                {
                    color_eyre::eyre::bail!("Could not create tab for window `{}`", window.title);
                }
            }
        }
    }

    Ok(())
}
