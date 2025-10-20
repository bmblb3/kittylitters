use std::hash::Hash;

use indexmap::IndexSet;

#[derive(Debug, Eq, Clone)]
pub struct Tab {
    pub id: Option<i64>,
    pub title: String,
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
    pub id: Option<i64>,
    pub title: String,
    pub cwd: Option<String>,
    pub cmd: Option<String>,
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
            windows: IndexSet::new(),
        }];
        let tab2 = [Tab {
            id: None,
            title: "A".to_string(),
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
            windows: IndexSet::new(),
        }];
        let tab2 = [Tab {
            id: Some(0),
            title: "B".to_string(),
            windows: IndexSet::new(),
        }];

        let set1: HashSet<_> = tab1.iter().collect();
        let set2: HashSet<_> = tab2.iter().collect();

        let diff: HashSet<_> = set1.difference(&set2).cloned().collect();
        dbg!(&diff);
        assert!(!diff.is_empty());
    }
}
