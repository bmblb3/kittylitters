use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use crate::collections::Tabs;

pub fn read_sess_yml<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Tabs, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: HashMap<String, HashMap<String, String>> = serde_yaml::from_reader(reader)?;

    let mut res_vec = Vec::new();
    for (tab, windows) in config {
        for (window, cwd) in windows {
            res_vec.push((tab.clone(), window, None, Some(cwd)));
        }
    }

    Ok(Tabs(res_vec))
}
