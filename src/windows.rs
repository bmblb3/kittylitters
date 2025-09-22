use std::fmt::Display;

use std::hash::Hasher;

use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Window {
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
        Self {
            title: title.into(),
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
}
