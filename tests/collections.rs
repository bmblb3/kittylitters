#[cfg(test)]
mod verify {
    use kittylitters::collections::Tabs;

    #[test]
    fn test_empty() {
        let tabs = Tabs(vec![]);
        assert!(tabs.verify());
    }

    #[test]
    fn test_one_tab_one_window() {
        let tabs = Tabs(vec![("T_X".to_string(), "W_A".to_string(), None)]);
        assert!(tabs.verify());
    }

    #[test]
    fn test_one_tab_two_windows() {
        let tabs = Tabs(vec![
            ("T_X".to_string(), "W_A".to_string(), None),
            ("T_X".to_string(), "W_B".to_string(), None),
        ]);
        assert!(tabs.verify());
    }

    #[test]
    fn test_two_tabs_one_window_each_same_name() {
        let tabs = Tabs(vec![
            ("T_X".to_string(), "W_A".to_string(), Some(1)),
            ("T_Y".to_string(), "W_B".to_string(), Some(2)),
        ]);
        assert!(tabs.verify());
    }

    #[test]
    fn test_one_tab_two_window_each_same_name() {
        let tabs = Tabs(vec![
            ("T_X".to_string(), "W_A".to_string(), Some(1)),
            ("T_X".to_string(), "W_A".to_string(), Some(2)),
        ]);
        assert!(!tabs.verify());
    }
}

mod differnce {
    use kittylitters::collections::Tabs;

    #[test]
    fn test_one_window_minus_zero_windows() {
        let tab1 = Tabs(vec![("T_X".to_string(), "W_A".to_string(), Some(1))]);
        let tab2 = Tabs(vec![]);

        let diff = tab1.sub(&tab2);
        assert_eq!(diff.0.len(), 1);
        assert_eq!(diff.0[0].0, "T_X");
        assert_eq!(diff.0[0].1, "W_A");
        assert_eq!(diff.0[0].2, Some(1));
    }

    #[test]
    fn test_sub_same() {
        let tab1 = Tabs(vec![("T_X".to_string(), "W_A".to_string(), Some(1))]);
        let tab2 = Tabs(vec![("T_X".to_string(), "W_A".to_string(), Some(1))]);
        let diff = tab1.sub(&tab2);
        assert_eq!(diff.0.len(), 0);
    }

    #[test]
    fn test_zero_windows_minux_one_window() {
        let tab1 = Tabs(vec![]);
        let tab2 = Tabs(vec![("T_X".to_string(), "W_A".to_string(), Some(1))]);
        let diff = tab1.sub(&tab2);
        assert_eq!(diff.0.len(), 0);
    }
}
