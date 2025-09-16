#[cfg(test)]
mod verify {
    use kittylitters::collections::{Tab, Tabs, Window};

    #[test]
    fn test_empty() {
        let tabs = Tabs(vec![]);
        assert!(tabs.verify());
    }

    #[test]
    fn test_one_tab_one_window() {
        let tabs = Tabs(vec![Tab {
            windows: vec![Window {
                id: None,
                title: "W_A".to_string(),
            }],
            title: "T_X".to_string(),
        }]);
        assert!(tabs.verify());
    }

    #[test]
    fn test_one_tab_two_windows() {
        let tabs = Tabs(vec![Tab {
            windows: vec![
                Window {
                    id: None,
                    title: "W_A".to_string(),
                },
                Window {
                    id: None,
                    title: "W_B".to_string(),
                },
            ],
            title: "T_X".to_string(),
        }]);
        assert!(tabs.verify());
    }

    #[test]
    fn test_two_tabs_one_window_each_same_name() {
        let tabs = Tabs(vec![
            Tab {
                windows: vec![Window {
                    id: Some(1),
                    title: "W_A".to_string(),
                }],
                title: "T_X".to_string(),
            },
            Tab {
                windows: vec![Window {
                    id: Some(2),
                    title: "W_A".to_string(),
                }],
                title: "T_Y".to_string(),
            },
        ]);
        assert!(tabs.verify());
    }

    #[test]
    fn test_one_tab_two_window_each_same_name() {
        let tabs = Tabs(vec![Tab {
            windows: vec![
                Window {
                    id: Some(1),
                    title: "W_A".to_string(),
                },
                Window {
                    id: Some(2),
                    title: "W_A".to_string(),
                },
            ],
            title: "T_X".to_string(),
        }]);
        assert!(!tabs.verify());
    }
}
