mod run_operations;

use kittylitters::collect_windows::collect_windows;
use kittylitters::first_value::get_first_values;
use kittylitters::read_yml::read_session_yml;
use kittylitters::set_logic::Solver;
use kittylitters::windows::Window;
use std::process::Command;

fn main() -> color_eyre::Result<()> {
    let current_windows = collect_windows()?;

    let home_dir = std::env::var("HOME").expect("HOME environment variable should be set");
    let config_path = format!("{}/.config/kitty/session.yml", home_dir);
    let desired_windows = read_session_yml(&config_path)
        .expect("Expect a valid yml file at ~/.config/kitty/session.yml");

    let current_tabs: Vec<Window> = Window::from_titles(&get_first_values(&current_windows));
    let desired_tabs: Vec<Window> = Window::from_instructions(&get_first_values(&desired_windows));
    let tab_operations = Solver::derive_operations(&current_tabs, &desired_tabs);
    run_operations::run_operations(tab_operations, run_operations::Kind::Tab)?;

    let current_windows = collect_windows()?;
    let current_windows: Vec<Window> = Window::from_titles(&current_windows);
    let desired_windows: Vec<Window> = Window::from_instructions(&desired_windows);
    let window_operations = Solver::derive_operations(&current_windows, &desired_windows);
    run_operations::run_operations(window_operations, run_operations::Kind::Window)?;

    Command::new("kitten")
        .args(["@", "goto-layout", "--match", "all", "stack"])
        .status()?;

    Ok(())
}
