use indexmap::IndexSet;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

pub struct Solver;

#[derive(Clone, Debug, PartialEq)]
pub enum Operation<T> {
    GoTo(T),
    MoveForward(usize),
    Close,
    New(T),
}

impl Solver {
    pub fn derive_operations<'a, T>(
        current_items: &'a [T],
        target_items: &'a [T],
    ) -> Vec<Operation<&'a T>>
    where
        T: Eq + Hash + Clone + Debug + Display,
    {
        let mut current_set: IndexSet<_> = current_items.iter().collect();
        let target_set: IndexSet<_> = target_items.iter().collect();

        let mut operations = Vec::new();
        for _ in 0..1000 {
            let snapshot = current_set.clone();
            let mut last_aligned_item = snapshot[0];

            let mut snapshot_iter = snapshot.iter();
            let mut target_iter = target_set.iter();

            loop {
                let snapshot_item = snapshot_iter.next();
                let target_item = target_iter.next();

                if snapshot_item.is_none() && target_item.is_none() {
                    break;
                }

                if snapshot_item == target_item {
                    last_aligned_item = snapshot_item.unwrap();
                    continue;
                }

                if let Some(target_item) = target_item
                    && !current_set.contains(target_item)
                {
                    operations.push(Operation::GoTo(last_aligned_item));
                    operations.push(Operation::New(target_item));
                    let insert_index = current_set
                        .get_index_of(last_aligned_item)
                        .expect("`last_aligned_item` should exist in `current_set`");
                    current_set.shift_insert(insert_index + 1, target_item);
                } else if let Some(current_item) = snapshot_item
                    && !target_set.contains(current_item)
                {
                    operations.push(Operation::GoTo(*current_item));
                    operations.push(Operation::Close);
                    current_set.shift_remove(current_item);
                } else if let Some(snapshot_item) = snapshot_item
                    && let Some(index) = current_set.get_index_of(snapshot_item)
                    && let Some(target_index) = target_set.get_index_of(snapshot_item)
                {
                    operations.push(Operation::GoTo(snapshot_item));
                    operations.push(Operation::MoveForward(target_index - index));
                    for i in index..target_index {
                        current_set.swap_indices(i, i + 1);
                    }
                }
                break;
            }

            if current_set.iter().eq(target_set.iter()) {
                break;
            }
        }

        operations
    }
}
