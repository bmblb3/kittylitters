#[cfg(test)]
mod verify {
    use kittylitters::collections::Windows;

    #[test]
    fn test_empty() {
        let tabs = Windows(vec![]);
        assert!(tabs.verify());
    }

    #[test]
    fn test_one_tab_one_window() {
        let tabs = Windows(vec![("T_X".to_string(), "W_A".to_string(), None, None)]);
        assert!(tabs.verify());
    }

    #[test]
    fn test_one_tab_two_windows() {
        let tabs = Windows(vec![
            ("T_X".to_string(), "W_A".to_string(), None, None),
            ("T_X".to_string(), "W_B".to_string(), None, None),
        ]);
        assert!(tabs.verify());
    }

    #[test]
    fn test_two_tabs_one_window_each_same_name() {
        let tabs = Windows(vec![
            ("T_X".to_string(), "W_A".to_string(), Some(1), None),
            ("T_Y".to_string(), "W_B".to_string(), Some(2), None),
        ]);
        assert!(tabs.verify());
    }

    #[test]
    fn test_one_tab_two_window_each_same_name() {
        let tabs = Windows(vec![
            ("T_X".to_string(), "W_A".to_string(), Some(1), None),
            ("T_X".to_string(), "W_A".to_string(), Some(2), None),
        ]);
        assert!(!tabs.verify());
    }
}
