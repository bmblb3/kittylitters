use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use std::fmt::Debug;

use crate::windows::Window;

pub struct Solver;

#[derive(Clone, Debug, PartialEq)]
pub enum Operation<T> {
    GoTo(T),
    MoveTabForward,
    MoveWindowForward,
    CloseWindow,
    NewWindow(T),
    NewTab(T),
}

impl Solver {
    pub fn derive_operations(
        current_windows: &[Window],
        desired_windows: &[Window],
    ) -> Vec<Operation<Window>> {
        let mut current_window_set: IndexSet<_> = current_windows.iter().collect();
        let desired_window_set: IndexSet<_> = desired_windows.iter().collect();

        let mut operations = Vec::new();
        for _ in 0..10 {
            let snapshot = current_window_set.clone();
            let mut last_aligned_window = snapshot[0];

            let mut snapshot_iter = snapshot.iter();
            let mut desired_iter = desired_window_set.iter();

            loop {
                let snapshot_window = snapshot_iter.next();
                let desired_window = desired_iter.next();

                if snapshot_window.is_none() && desired_window.is_none() {
                    break;
                }

                if let Some(snapshot_window) = snapshot_window
                    && let Some(desired_window) = desired_window
                    && snapshot_window == desired_window
                {
                    last_aligned_window = snapshot_window;
                    continue;
                }

                if let Some(desired_window) = desired_window
                    && !current_window_set.contains(desired_window)
                {
                    operations.push(Operation::GoTo(last_aligned_window.clone()));
                    if last_aligned_window.tab_title == desired_window.tab_title {
                        operations.push(Operation::NewWindow(desired_window.to_owned().clone()));
                    } else {
                        operations.push(Operation::NewTab(desired_window.to_owned().clone()));
                    }
                    let insert_index = current_window_set
                        .get_index_of(last_aligned_window)
                        .expect("`last_aligned_window` should exist in `current_window_set`");
                    current_window_set.shift_insert(insert_index + 1, desired_window);
                } else if let Some(current_window) = snapshot_window
                    && !desired_window_set.contains(current_window)
                {
                    operations.push(Operation::GoTo(current_window.to_owned().clone()));
                    operations.push(Operation::CloseWindow);
                    current_window_set.shift_remove(current_window);
                } else if let Some(snapshot_window) = snapshot_window
                    && let Some(current_index) = current_window_set.get_index_of(snapshot_window)
                    && let Some(desired_window_here) = desired_window_set.get_index(current_index)
                    && let Some(destination_index) =
                        desired_window_set.get_index_of(snapshot_window)
                {
                    operations.push(Operation::GoTo(snapshot_window.to_owned().clone()));
                    if snapshot_window.tab_title != desired_window_here.tab_title {
                        // tab
                        let snapshot_indices_groupedby_tab = current_window_set
                            .iter()
                            .chunk_by(|window| window.tab_title.to_string())
                            .into_iter()
                            .map(|(tab_title, group)| (tab_title, group.collect::<Vec<_>>()))
                            .collect::<IndexMap<String, Vec<_>>>();

                        let desired_indices_groupedby_tab = desired_window_set
                            .iter()
                            .chunk_by(|window| window.tab_title.to_string())
                            .into_iter()
                            .map(|(tab_title, group)| (tab_title, group.collect::<Vec<_>>()))
                            .collect::<IndexMap<String, Vec<_>>>();

                        let current_tab_pos = snapshot_indices_groupedby_tab
                            .get_index_of(&snapshot_window.tab_title)
                            .expect("tab should be present in group");
                        let desired_tab_pos = desired_indices_groupedby_tab
                            .get_index_of(&snapshot_window.tab_title)
                            .expect("tab should be present in group");

                        let mut rearranged_tabs = snapshot_indices_groupedby_tab.clone();
                        for i in current_tab_pos..desired_tab_pos {
                            operations.push(Operation::MoveTabForward);
                            rearranged_tabs.swap_indices(i, i + 1);
                        }

                        current_window_set = rearranged_tabs
                            .values()
                            .flatten()
                            .cloned()
                            .cloned()
                            .collect::<IndexSet<_>>();
                    } else {
                        // window
                        for i in current_index..destination_index {
                            operations.push(Operation::MoveWindowForward);
                            current_window_set.swap_indices(i, i + 1);
                        }
                    }
                }
                break;
            }

            if current_window_set.iter().eq(desired_window_set.iter()) {
                break;
            }
        }

        operations
    }
}
