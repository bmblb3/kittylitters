use std::process::Command;

use indexmap::IndexSet;

use crate::OsWindow;
use crate::Tab;
use crate::Window;

pub fn ls() -> anyhow::Result<IndexSet<OsWindow>> {
    let kitten_ls = serde_json::from_str::<serde_json::Value>(&String::from_utf8(
        Command::new("kitten").args(["@", "ls"]).output()?.stdout,
    )?)?;
    let os_window_array = kitten_ls
        .as_array()
        .expect("kitten @ ls should parse to array");

    let mut existing_os_windows: IndexSet<OsWindow> = IndexSet::new();
    for os_window in os_window_array {
        let os_window_id = os_window
            .get("id")
            .expect("`os_window` should have `id`")
            .as_i64()
            .expect("`os_window[id]` should parse to an i64");

        let os_window_is_active = os_window
            .get("is_active")
            .expect("`os_window` should have `is_active`")
            .as_bool()
            .expect("`os_window[is_active]` should parse to a bool");

        let mut existing_os_window = OsWindow {
            id: os_window_id as usize,
            is_active: os_window_is_active,
            tabs: IndexSet::new(),
        };

        let tabs_array = os_window
            .get("tabs")
            .expect("The active os_window should have `tabs`")
            .as_array()
            .expect("`tabs` should parse to an array");

        for tab in tabs_array {
            let tab_id = tab
                .get("id")
                .expect("`tab` should have `id`")
                .as_i64()
                .expect("`tab[id]` should parse to an i64");

            let tab_title = tab
                .get("title")
                .expect("`tab` should have `title`")
                .as_str()
                .expect("`tab[title]` should parse to an str");

            let tab_is_active = tab
                .get("is_active")
                .expect("`tab` should have `is_active`")
                .as_bool()
                .expect("`tab[is_active]` should parse to a bool");

            let windows_array = tab
                .get("windows")
                .expect("`tabs` should contain `windows`")
                .as_array()
                .expect("`windows` should parse to an array");

            let mut existing_tab = Tab {
                id: Some(tab_id as usize),
                title: tab_title.to_string(),
                is_active: Some(tab_is_active),
                windows: IndexSet::new(),
            };

            for window in windows_array {
                let window_title = window
                    .get("title")
                    .expect("`window` should have `title`")
                    .as_str()
                    .expect("`window[title]` should parse to an str");

                let window_id = window
                    .get("id")
                    .expect("`window` should have `id`")
                    .as_i64()
                    .expect("`window[id]` should parse to an i64");

                let window_is_active = window
                    .get("is_active")
                    .expect("`window` should have `is_active`")
                    .as_bool()
                    .expect("`window[is_active]` should parse to a bool");

                let existing_window = Window {
                    id: Some(window_id as usize),
                    title: window_title.to_string(),
                    is_active: Some(window_is_active),
                    cmd: None,
                    cwd: None,
                };
                existing_tab.windows.insert(existing_window);
            }
            existing_os_window.tabs.insert(existing_tab);
        }
        existing_os_windows.insert(existing_os_window);
    }
    Ok(existing_os_windows)
}
