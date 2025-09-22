use std::process::Command;

use crate::collections::Windows;

pub fn remove_windows(tabs: Windows) {
    for (_, _, winid, _) in tabs.0 {
        Command::new("kitten")
            .args([
                "@",
                "close-window",
                "--match",
                &format!(
                    "id:{}",
                    winid.ok_or("Cannot remove a window with ID None").unwrap()
                ),
            ])
            .output()
            .unwrap();
    }
}
