use std::fs::File;
use std::io::BufReader;

use indexmap::IndexSet;

use crate::Tab;
use crate::Window;

pub fn read_session_yml<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<IndexSet<Tab>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let yaml: serde_yaml::Value = serde_yaml::from_reader(reader)?;

    let mut desired_tabs = IndexSet::new();
    if let serde_yaml::Value::Mapping(tabs) = yaml {
        for (tab_title, windows) in tabs {
            let tab_title = tab_title.as_str().expect("Tab key should be a string");
            let mut desired_tab = Tab {
                id: None,
                title: tab_title.to_string(),
                is_active: None,
                windows: IndexSet::new(),
            };

            if let serde_yaml::Value::Mapping(windows) = windows {
                for (window_title, window_instruction) in windows {
                    let window_title = window_title
                        .as_str()
                        .expect("Window key should be a string");
                    if let serde_yaml::Value::Mapping(window_instruction) = window_instruction {
                        let cwd = window_instruction
                            .get("cwd")
                            .and_then(|val| val.as_str().map(|str| str.to_string()));
                        let cmd = window_instruction
                            .get("cmd")
                            .and_then(|val| val.as_str().map(|str| str.to_string()));

                        desired_tab.windows.insert(Window {
                            id: None,
                            title: window_title.to_string(),
                            is_active: None,
                            cwd,
                            cmd,
                        });
                    }
                }
                desired_tabs.insert(desired_tab);
            }
        }
    }

    Ok(desired_tabs)
}
