use std::process::Command;

use serde_json::Value;

use crate::collections::Tabs;

pub fn collect_windows() -> Result<Tabs, Box<dyn std::error::Error>> {
    let output = Command::new("kitten").args(["@", "ls"]).output()?;

    let stdout = output.stdout;

    let stdout_str = String::from_utf8(stdout)?;

    let stdout_value = serde_json::from_str::<Value>(&stdout_str)?;

    let session_array = stdout_value
        .as_array()
        .ok_or("Could not convert JSON into session array")?;

    let focused_session = session_array
        .iter()
        .find(|sess| {
            sess.get("is_focused")
                .is_some_and(|is_focused| is_focused.as_bool().is_some_and(|is_focused| is_focused))
        })
        .ok_or("No session is focused")?;

    let tabs = focused_session
        .get("tabs")
        .ok_or("Focused session does not contain `tabs`")?;

    let tabs_array = tabs
        .as_array()
        .ok_or("Tabs could not be converted to array")?;

    let mut res_vec = Vec::new();

    for tab in tabs_array {
        let tab_title = tab
            .get("title")
            .ok_or("Tab does not contain `title`")?
            .as_str()
            .ok_or("Tab title could not be converted to string")?;
        let windows = tab.get("windows").ok_or("Tab does not contain `windows`")?;
        let windows_array = windows
            .as_array()
            .ok_or("Windows could not be converted to array")?;
        for window in windows_array {
            let window_title = window
                .get("title")
                .ok_or("Window does not contain `title`")?
                .as_str()
                .ok_or("Window title could not be converted to string")?;
            let window_id = window
                .get("id")
                .ok_or("Window does not contain `id`")?
                .as_u64()
                .ok_or("Window id could not be converted to u64")?;
            res_vec.push((
                String::from(tab_title),
                String::from(window_title),
                Some(window_id as u8),
                None,
            ));
        }
    }

    Ok(Tabs(res_vec))
}
