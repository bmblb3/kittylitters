use indexmap::IndexMap;
use std::fs::File;
use std::io::BufReader;

type CascadeType = IndexMap<String, IndexMap<String, IndexMap<String, String>>>;

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
                        let cwd_dflt = serde_yaml::Value::from("~");
                        let cwd = window_instruction
                            .get("cwd")
                            .unwrap_or(&cwd_dflt)
                            .as_str()
                            .expect("`cwd` should be a string");
                        let cmd_dflt = serde_yaml::Value::from("");
                        let cmd = window_instruction
                            .get("cmd")
                            .unwrap_or(&cmd_dflt)
                            .as_str()
                            .expect("`cmd` should be a string");

                        let mut window_mapping = IndexMap::new();
                        window_mapping.insert("cwd".to_string(), cwd.to_string());
                        window_mapping.insert("cmd".to_string(), cmd.to_string());

                        cascade
                            .entry(tab_title.to_string())
                            .or_default()
                            .insert(window_title.to_string(), window_mapping);
                    }
                }
            }
        }
    }

    Ok(cascade)
}
