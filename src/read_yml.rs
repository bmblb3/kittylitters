use indexmap::IndexMap;
use std::fs::File;
use std::io::BufReader;

type CascadeType = IndexMap<String, Vec<IndexMap<String, Option<String>>>>;

pub fn read_session_yml<P: AsRef<std::path::Path>>(path: P) -> color_eyre::Result<CascadeType> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let yaml: serde_yaml::Value = serde_yaml::from_reader(reader)?;

    let mut cascade: CascadeType = IndexMap::new();

    if let serde_yaml::Value::Mapping(tabs) = yaml {
        for (tab_title, windows) in tabs {
            let tab_title = tab_title.as_str().expect("Tab key should be a string");
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

                        let mut window_mapping = IndexMap::new();
                        window_mapping.insert("title".to_string(), Some(window_title.to_string()));
                        window_mapping.insert("cwd".to_string(), cwd);
                        window_mapping.insert("cmd".to_string(), cmd);

                        cascade
                            .entry(tab_title.to_string())
                            .or_default()
                            .push(window_mapping);
                    }
                }
            }
        }
    }

    Ok(cascade)
}
