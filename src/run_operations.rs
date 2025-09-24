use std::process::Command;

use kittylitters::windows::Window;

use kittylitters::set_logic::Operation;

pub enum Kind {
    Window,
    Tab,
}

pub fn run_operations(
    operations: Vec<Operation<&Window>>,
    window_kind: Kind,
) -> color_eyre::Result<()> {
    let window_kind = match window_kind {
        Kind::Window => "window",
        Kind::Tab => "tab",
    };

    for operation in operations {
        match operation {
            Operation::GoTo(window) => {
                Command::new("kitten")
                    .args([
                        "@",
                        "focus-window",
                        "--match",
                        &format!("title:^{}$", window.title),
                    ])
                    .status()?;
            }
            Operation::CloseWindow => {
                Command::new("kitten")
                    .args(["@", "action", "close_window"])
                    .status()?;
            }
            Operation::MoveWindowForward(count) => {
                let action = "move_".to_owned() + window_kind + "_forward";
                for _ in 0..count {
                    Command::new("kitten")
                        .args(["@", "action", action.as_str()])
                        .status()?;
                }
            }
            Operation::NewWindow(window) => {
                let mut args = vec![
                    "@".to_string(),
                    "launch".to_string(),
                    "--type".to_string(),
                    window_kind.to_string(),
                    "--title".to_string(),
                    window.title.to_string(),
                ];
                if let Some(cwd) = &window.cwd {
                    args.push("--cwd".to_string());
                    args.push(cwd.clone());
                }
                Command::new("kitten").args(args).status()?;
            }
        }
    }

    Ok(())
}
