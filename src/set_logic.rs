use indexmap::IndexSet;
use std::fmt::Debug;

use crate::windows::Window;

pub struct Solver;

#[derive(Clone, Debug, PartialEq)]
pub enum Operation<T> {
    GoTo(T),
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
        for _ in 0..1000 {
            let snapshot = current_window_set.clone();
            let mut last_aligned_window = snapshot[0];

            let mut snapshot_iter = snapshot.iter();
            let mut desired_iter = desired_window_set.iter();

            loop {
                let snapshot_item = snapshot_iter.next();
                let desired_item = desired_iter.next();

                if snapshot_item.is_none() && desired_item.is_none() {
                    break;
                }

                if snapshot_item == desired_item {
                    last_aligned_window = snapshot_item.unwrap();
                    continue;
                }

                if let Some(target_item) = desired_item
                    && !current_window_set.contains(target_item)
                {
                    operations.push(Operation::GoTo(last_aligned_window.clone()));
                    if last_aligned_window.tab_title == target_item.tab_title {
                        operations.push(Operation::NewWindow(target_item.to_owned().clone()));
                    } else {
                        operations.push(Operation::NewTab(target_item.to_owned().clone()));
                    }
                    let insert_index = current_window_set
                        .get_index_of(last_aligned_window)
                        .expect("`last_aligned_item` should exist in `current_set`");
                    current_window_set.shift_insert(insert_index + 1, target_item);
                } else if let Some(current_item) = snapshot_item
                    && !desired_window_set.contains(current_item)
                {
                    operations.push(Operation::GoTo(current_item.to_owned().clone()));
                    operations.push(Operation::CloseWindow);
                    current_window_set.shift_remove(current_item);
                } else if let Some(snapshot_item) = snapshot_item
                    && let Some(index) = current_window_set.get_index_of(snapshot_item)
                    && let Some(target_index) = desired_window_set.get_index_of(snapshot_item)
                {
                    operations.push(Operation::GoTo(snapshot_item.to_owned().clone()));
                    for i in index..target_index {
                        operations.push(Operation::MoveWindowForward);
                        current_window_set.swap_indices(i, i + 1);
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
