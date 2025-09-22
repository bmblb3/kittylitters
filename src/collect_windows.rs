use std::process::Command;

use indexmap::IndexMap;

type CascadeType = IndexMap<String, Vec<String>>;

pub fn collect_windows() -> color_eyre::Result<CascadeType> {
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

    let mut cascade: CascadeType = IndexMap::new();

    for tab in tabs_array {
        let windows_array = tab
            .get("windows")
            .expect("`tabs` should contain `windows`")
            .as_array()
            .expect("`windows` should parse to an array");

        for window in windows_array {
            let title = window
                .get("title")
                .expect("`window` should have `title`")
                .as_str()
                .expect("`window[title]` should parse to a String");

            let (tab_title, window_title) = title
                .split_once("/")
                .expect("Existing window titles should be of the form '<tab>/<window>'");

            cascade
                .entry(tab_title.to_string())
                .or_default()
                .push(window_title.to_string());
        }
    }

    Ok(cascade)
}
