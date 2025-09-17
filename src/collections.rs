use std::collections::HashSet;

pub struct Tabs(pub Vec<(String, String, Option<u8>, Option<String>)>);

impl Tabs {
    pub fn verify(&self) -> bool {
        let mut flatvec = Vec::new();
        for (tab_title, window_title, _, _) in &self.0 {
            flatvec.push((tab_title, window_title));
        }
        let flatset: HashSet<_> = flatvec.iter().collect();

        flatvec.len() == flatset.len()
    }

    pub fn sub(&self, other: &Self) -> Self {
        let mut other_flatvec = Vec::new();
        for (tab_title, window_title, _, _) in &other.0 {
            other_flatvec.push((tab_title, window_title));
        }
        let other_flatset: HashSet<_> = other_flatvec.iter().collect();

        let mut result_vec = Vec::new();
        for (tab_title, window_title, window_id, cwd) in &self.0 {
            let combined = (tab_title, window_title);
            if !other_flatset.contains(&combined) {
                result_vec.push((
                    tab_title.clone(),
                    window_title.clone(),
                    *window_id,
                    cwd.clone(),
                ));
            }
        }
        Tabs(result_vec)
    }
}
