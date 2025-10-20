use std::process::Command;

use kittylitters::Operations::*;
use kittylitters::OsWindow;
use kittylitters::Tab;

fn main() -> anyhow::Result<()> {
    let home_dir = std::env::var("HOME").expect(
        "HOME environment variable should
    // be set",
    );
    let config_path = format!("{}/.config/kitty/session.yml", home_dir);

    // let editor = if let Ok(e) = std::env::var("EDITOR") {
    //     e
    // } else {
    //     "vi".to_string()
    // };
    // Command::new(editor).arg(&config_path).status()?;

    let desired_tabs = kittylitters::read_session_yml(&config_path)?;

    // let current_tabs = kittylitters::ls()?;
    // let mut _active_tab = None;
    // let mut _active_window = None;
    // for tab in current_tabs.clone() {
    //     if tab.is_active.is_some_and(|bool| bool) {
    //         for window in &tab.windows {
    //             if window.is_active.is_some_and(|bool| bool) {
    //                 _active_tab = Some(tab.clone());
    //                 _active_window = Some(window.clone());
    //                 let args = ["@", "detach-window"];
    //                 Command::new("kitten").args(args).output()?;
    //             }
    //         }
    //     }
    // }

    let mut current_tabs = collect_tabs_from_active_os_window();

    let tab_ops = kittylitters::set_operations(current_tabs.clone(), desired_tabs.clone());
    for op in tab_ops {
        match op {
            GoTo(t) => {
                let tab_id = current_tabs
                    .get(&t)
                    .expect("`tab` should exist in `current_tabs`")
                    .id
                    .expect("an item from `current_tabs` should have an `id`");
                let match_str = format!("id:{}", tab_id);
                let args = ["@", "focus-tab", "--match", match_str.as_str()];
                Command::new("kitten").args(args).output()?;
            }
            Create(t) => {
                let args = [
                    "@",
                    "launch",
                    "--type",
                    "tab",
                    "--tab-title",
                    &t.title,
                    "--title",
                    "TMPWINDOW",
                ];
                Command::new("kitten").args(args).output()?;
                current_tabs = collect_tabs_from_active_os_window();
            }
            Close => {
                let args = ["@", "close-tab"];
                Command::new("kitten").args(args).output()?;
            }
            MoveForward => {
                let args = ["@", "action", "move_tab_forward"];
                Command::new("kitten").args(args).output()?;
            }
        }
    }

    let mut current_tabs = collect_tabs_from_active_os_window();

    for current_tab in current_tabs.clone() {
        let desired_tab = desired_tabs.get(&current_tab).expect(
            "`desired_tabs` should have every `tab` from `current_tabs` since they should be \
             aligned by now",
        );
        let current_windows = current_tab.clone().windows;
        let desired_windows = desired_tab.clone().windows;

        let window_ops = kittylitters::set_operations(current_windows.clone(), desired_windows);
        for op in window_ops {
            match op {
                GoTo(w) => {
                    let win_id = current_tabs
                        .get(&current_tab)
                        .expect("`current_tab` should exist in `current_tabtabs`")
                        .windows
                        .get(&w)
                        .expect("`win` should exist in `current_windows`")
                        .id
                        .expect("an item from `current_windows` should have an `id`");
                    let match_str = format!("id:{}", win_id);
                    let args = ["@", "focus-window", "--match", match_str.as_str()];
                    Command::new("kitten").args(args).output()?;
                }
                Create(w) => {
                    let args = ["@", "launch", "--type", "window", "--title", &w.title];
                    Command::new("kitten").args(args).output()?;
                    current_tabs = collect_tabs_from_active_os_window();
                }
                Close => {
                    let args = ["@", "close-window"];
                    Command::new("kitten").args(args).output()?;
                }
                MoveForward => {
                    let args = ["@", "action", "move_window_forward"];
                    Command::new("kitten").args(args).output()?;
                }
            }
        }
    }

    Command::new("kitten")
        .args(["@", "goto-layout", "--match", "all", "stack"])
        .status()?;

    Ok(())
}

fn collect_tabs_from_active_os_window() -> indexmap::IndexSet<Tab> {
    kittylitters::ls()
        .expect("`kitty @ ls` should work")
        .iter()
        .find(|ow| ow.is_active)
        .cloned()
        .expect("There must be exactly one active OS Window")
        .tabs
}
