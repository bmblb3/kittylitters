use std::collections::HashSet;

pub struct Window {
    pub id: Option<u8>,
    pub title: String,
}

pub struct Tab {
    pub windows: Vec<Window>,
    pub title: String,
}

pub struct Tabs(pub Vec<Tab>);

impl Tabs {
    pub fn verify(&self) -> bool {
        let mut flatvec = Vec::new();
        for tab in &self.0 {
            let this_tab = &tab.title;
            for win in &tab.windows {
                let this_win = &win.title;
                let combined = (this_tab, this_win);
                flatvec.push(combined);
            }
        }

        let flatset: HashSet<_> = flatvec.iter().collect();

        flatvec.len() == flatset.len()
    }
}
