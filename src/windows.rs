use std::fmt::Display;

use std::hash::Hasher;

use std::hash::Hash;

use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct Window {
    pub tab_title: String,
    pub title: String,
    pub cwd: Option<String>,
    pub cmd: Option<String>,
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

impl Eq for Window {}

impl Hash for Window {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.title.hash(state);
    }
}

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Window {
    pub fn new(title: impl Into<String>) -> Self {
        let title = title.into();
        let tab_title = title.clone().split('/').next().expect("").to_string();
        Self {
            tab_title,
            title,
            cwd: None,
            cmd: None,
        }
    }

    pub fn with_cwd(mut self, cwd: impl Into<Option<String>>) -> Self {
        self.cwd = cwd.into();
        self
    }

    pub fn with_cmd(mut self, cmd: impl Into<Option<String>>) -> Self {
        self.cmd = cmd.into();
        self
    }

    pub fn titles_to_windows(tab_title: &String, window_title: &String) -> Self {
        let window_title = if window_title.is_empty() {
            window_title.to_string()
        } else {
            format!("/{window_title}")
        };
        Window::new(format!("{0}{1}", tab_title, window_title))
    }

    pub fn from_titles(windows: &IndexMap<String, Vec<String>>) -> Vec<Self> {
        windows
            .iter()
            .flat_map(|(tab_title, window_titles)| {
                window_titles
                    .iter()
                    .map(|window_title| Self::titles_to_windows(tab_title, window_title))
                    .collect::<Vec<Window>>()
            })
            .collect()
    }

    pub fn from_instructions(
        windows: &IndexMap<String, Vec<IndexMap<String, Option<String>>>>,
    ) -> Vec<Self> {
        windows
            .iter()
            .flat_map(|(tab_title, window_instructions)| {
                window_instructions
                    .iter()
                    .map(|window_instruction| {
                        let window_title = window_instruction
                            .get("title")
                            .and_then(|x| x.as_ref())
                            .expect("Window instruction should have 'title'");
                        let window_cwd = window_instruction.get("cwd").and_then(|x| x.as_ref());
                        let window_cmd = window_instruction.get("cmd").and_then(|x| x.as_ref());
                        Self::titles_to_windows(tab_title, window_title)
                            .with_cwd(window_cwd.map(|s| s.to_owned()))
                            .with_cmd(window_cmd.map(|s| s.to_owned()))
                    })
                    .collect::<Vec<Window>>()
            })
            .collect()
    }
}
