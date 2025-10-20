use std::process::Command;

use indexmap::IndexSet;

use crate::Tab;
use crate::Window;

pub fn collect_windows() -> color_eyre::Result<IndexSet<Tab>> {
    let kitten_ls = serde_json::from_str::<serde_json::Value>(&String::from_utf8(
        Command::new("kitten").args(["@", "ls"]).output()?.stdout,
    )?)?;
    let session_array = kitten_ls
        .as_array()
        .expect("kitten @ ls should parse to array");

    let focused_session = session_array
        .iter()
        .find(|sess| {
            sess.get("is_focused")
                .is_some_and(|is_focused| is_focused.as_bool().is_some_and(|is_focused| is_focused))
        })
        .expect("A session should be focused");

    let tabs_array = focused_session
        .get("tabs")
        .expect("The focused session should have `tabs`")
        .as_array()
        .expect("`tabs` should parse to an array");

    let mut existing_tabs: IndexSet<Tab> = IndexSet::new();
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
            id: Some(tab_id),
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
                id: Some(window_id),
                title: window_title.to_string(),
                is_active: Some(window_is_active),
                cmd: None,
                cwd: None,
            };
            existing_tab.windows.insert(existing_window);
        }
        existing_tabs.insert(existing_tab);
    }
    Ok(existing_tabs)
}
