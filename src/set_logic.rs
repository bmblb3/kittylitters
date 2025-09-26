use indexmap::IndexSet;
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
                    let snapshot_tab = snapshot_item.tab_title.clone();
                    let desired_tab = desired_window_set
                        .get_index(index)
                        .unwrap()
                        .tab_title
                        .clone();

                    operations.push(Operation::GoTo(snapshot_item.to_owned().clone()));
                    if snapshot_tab != desired_tab {
                        // tab
                        let mut next_tab_indices = vec![index];
                        let mut i = index;
                        while let Some(this_window) = current_window_set.get_index(i)
                            && let Some(next_window) = current_window_set.get_index(i + 1)
                        {
                            if this_window.tab_title == next_window.tab_title {
                                i += 1;
                                continue;
                            }

                            if this_window.tab_title == desired_tab {
                                break;
                            }

                            operations.push(Operation::MoveTabForward);
                            next_tab_indices.push(i + 1);
                            i += 1;
                        }
                        next_tab_indices.push(i + 1);

                        let w: Vec<&[usize]> = next_tab_indices.windows(2).collect();
                        let (left, right) = w.split_at(1);
                        let shifted = [right.to_vec(), left.to_vec()].concat();
                        let ashifted = shifted
                            .iter()
                            .flat_map(|x| (x[0]..x[1]).collect::<Vec<_>>())
                            .collect::<Vec<usize>>();
                        dbg!(&ashifted);

                        let mut bshifted = ashifted.iter();

                        dbg!(&current_window_set);
                        let cloned = current_window_set.clone();
                        let mut i = index;
                        while let Some(i2) = bshifted.next()
                            && let Some(item) = cloned.get_index(*i2)
                        {
                            current_window_set.shift_insert(i, item);
                            i += 1;
                        }
                        dbg!(&current_window_set);
                    } else {
                        // window
                        for i in index..target_index {
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
