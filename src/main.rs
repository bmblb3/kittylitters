mod run_operations;

use std::process::Command;

use kittylitters::collect_windows::collect_windows;
use kittylitters::read_yml::read_session_yml;
use kittylitters::set_logic::Solver;
use kittylitters::windows::Window;

fn main() -> color_eyre::Result<()> {
    let current_windows = collect_windows()?;
    let current_windows: Vec<Window> = Window::from_titles(&current_windows);

    let home_dir = std::env::var("HOME").expect("HOME environment variable should be set");
    let config_path = format!("{}/.config/kitty/session.yml", home_dir);

    let editor = if let Ok(e) = std::env::var("EDITOR") {
        e
    } else {
        "vi".to_string()
    };
    Command::new(editor).arg(&config_path).status()?;

    let desired_windows = read_session_yml(&config_path)
        .expect("Expect a valid yml file at ~/.config/kitty/session.yml");
    let desired_windows: Vec<Window> = Window::from_instructions(&desired_windows);

    let window_operations = Solver::derive_operations(&current_windows, &desired_windows);
    run_operations::run_operations(window_operations)?;

    Command::new("kitten")
        .args(["@", "goto-layout", "--match", "all", "stack"])
        .status()?;

    Ok(())
}
