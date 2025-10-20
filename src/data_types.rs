use std::hash::Hash;

use indexmap::IndexSet;

#[derive(Debug, Eq, Clone)]
pub struct OsWindow {
    pub id: usize,
    pub is_active: bool,
    pub tabs: IndexSet<Tab>,
}
impl Hash for OsWindow {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialEq for OsWindow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Tab {
    pub id: Option<usize>,
    pub title: String,
    pub is_active: Option<bool>,
    pub windows: IndexSet<Window>,
}
impl Hash for Tab {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title.hash(state);
    }
}
impl PartialEq for Tab {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Window {
    pub id: Option<usize>,
    pub title: String,
    pub is_active: Option<bool>,
    pub cwd: Option<String>,
    pub cmd: Option<Vec<String>>,
}
impl Hash for Window {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title.hash(state);
    }
}
impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_diff_empty_when_titles_same() {
        let tab1 = [Tab {
            id: Some(0),
            title: "A".to_string(),
            is_active: Some(true),
            windows: IndexSet::new(),
        }];
        let tab2 = [Tab {
            id: None,
            title: "A".to_string(),
            is_active: Some(false),
            windows: IndexSet::new(),
        }];

        let set1: HashSet<_> = tab1.iter().collect();
        let set2: HashSet<_> = tab2.iter().collect();

        let diff: HashSet<_> = set1.difference(&set2).cloned().collect();
        dbg!(&diff);
        assert!(diff.is_empty());
    }

    #[test]
    fn test_diff_nonempty_when_titles_different() {
        let tab1 = [Tab {
            id: Some(0),
            title: "A".to_string(),
            is_active: Some(true),
            windows: IndexSet::new(),
        }];
        let tab2 = [Tab {
            id: Some(0),
            title: "B".to_string(),
            is_active: Some(true),
            windows: IndexSet::new(),
        }];

        let set1: HashSet<_> = tab1.iter().collect();
        let set2: HashSet<_> = tab2.iter().collect();

        let diff: HashSet<_> = set1.difference(&set2).cloned().collect();
        dbg!(&diff);
        assert!(!diff.is_empty());
    }
}
