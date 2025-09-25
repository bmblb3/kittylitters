use std::process::Command;

use kittylitters::windows::Window;

use kittylitters::set_logic::Operation;

pub fn run_operations(operations: Vec<Operation<Window>>) -> color_eyre::Result<()> {
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
            Operation::MoveWindowForward => {
                Command::new("kitten")
                    .args(["@", "action", "move_window_forward"])
                    .status()?;
            }
            Operation::NewWindow(ref window) | Operation::NewTab(ref window) => {
                let kind = if let Operation::NewWindow(_) = operation {
                    "window"
                } else {
                    "tab"
                };

                let mut args = vec![
                    "@".to_string(),
                    "launch".to_string(),
                    "--type".to_string(),
                    kind.to_string(),
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
