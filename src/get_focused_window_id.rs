use serde_json::Value;

use serde_json::from_str;

use std::process::Command;

pub fn get_focused_window_id() -> Result<u8, Box<dyn std::error::Error>> {
    let output = Command::new("kitten").args(["@", "ls"]).output()?;

    let stdout = String::from_utf8(output.stdout)?;

    let stdout_str = from_str::<Value>(&stdout)?;

    let array = stdout_str
        .as_array()
        .ok_or("Could not convert sessions to array")?;

    let focused_session = array
        .iter()
        .find(|sess| {
            sess.get("is_focused")
                .is_some_and(|f| f.as_bool().is_some_and(|f| f))
        })
        .ok_or("No session is focused")?;

    let tabs = focused_session.get("tabs").ok_or("Could not get tabs")?;

    let focused_tab = tabs
        .as_array()
        .ok_or("Could not convert tabs to array")?
        .iter()
        .find(|tab| {
            tab.get("is_focused")
                .is_some_and(|f| f.as_bool().is_some_and(|f| f))
        })
        .ok_or("No tab is focused")?;

    let windows = focused_tab.get("windows").ok_or("Could not get windows")?;

    let focused_window = windows
        .as_array()
        .ok_or("Could not convert windows to array")?
        .iter()
        .find(|window| {
            window
                .get("is_focused")
                .is_some_and(|f| f.as_bool().is_some_and(|f| f))
        })
        .ok_or("No window is focused")?;

    let win_id = focused_window.get("id").ok_or("Could not get id")?;

    let retval = win_id.as_u64().ok_or("Could not convert to unsigned int")?;

    Ok(retval as u8)
}
