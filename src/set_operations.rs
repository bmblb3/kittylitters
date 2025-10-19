use std::fmt::Debug;
use std::hash::Hash;

use indexmap::IndexSet;

pub fn set_operations<T>(
    mut current_set: IndexSet<T>,
    desired_set: IndexSet<T>,
) -> Vec<Operations<T>>
where
    T: Eq + Hash + Copy + Debug,
{
    let mut operations = Vec::new();
    for _ in 1..5 {
        if desired_set == current_set {
            break;
        }

        let cs = current_set.clone();
        let mut current_iter = cs.iter();
        let mut desired_iter = desired_set.iter();

        for i in 0..5 {
            let this_item = current_iter.next().cloned();
            let target_item = desired_iter.next().cloned();

            if this_item.is_none() && target_item.is_none() {
                break;
            }

            if let Some(this_item) = this_item
                && let Some(target_item) = &target_item
                && &this_item == target_item
            {
                continue;
            }

            if let Some(target_item) = &target_item
                && !&current_set.contains(target_item)
            {
                operations.push(Operations::GoTo(get_prev_item(&current_set, i)));
                operations.push(Operations::Create(*target_item));
                current_set.shift_insert((i + 1).min(current_set.len()), *target_item);
            }

            if let Some(this_item) = &this_item
                && !&desired_set.contains(this_item)
            {
                operations.push(Operations::GoTo(*this_item));
                operations.push(Operations::Close);
                current_set.shift_remove_index(i);
            }
        }
    }
    operations
}

fn get_prev_item<T>(current_set: &IndexSet<T>, i: usize) -> T
where
    T: Copy,
{
    let prev_id = i.saturating_sub(1).min(current_set.len() - 1);
    *current_set.get_index(prev_id).expect(
        "`current_set` should be indexed by `prev_id`, which is is
inside the index range",
    )
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operations<T> {
    GoTo(T),
    Create(T),
    Close,
}

#[cfg(test)]
mod tests {
    use super::Operations;

    #[rstest::rstest]
    #[case(
        &["A"],
        &["A"],
        &[]
    )]
    #[case(
        &["A"],
        &["A", "B"],
        &[
            Operations::GoTo("A"),  // ["A",   ]
            Operations::Create("B") // [ A , B ]
        ]
    )]
    #[case(
        &["A", "B"],
        &["B"],
        &[
            Operations::GoTo("A"), // ["A", B ]
            Operations::Close      // [ B ,   ]
        ]
    )]
    #[case(
        &["A", "B"],
        &["A"],
        &[
            Operations::GoTo("B"), // [ A ,"B"]
            Operations::Close      // [ B ,   ]
        ]
    )]
    #[case(
        &["A"],
        &["B"],
        &[
            Operations::GoTo("A"),   // ["A",   ]
            Operations::Create("B"), // [ A , B ]
            Operations::GoTo("A"),   // ["A", B ]
            Operations::Close,       // [   , B ]
        ]
    )]
    #[case(
        &["A", "B"],
        &["C"],
        &[
            Operations::GoTo("A"),   // ["A",   , B ]
            Operations::Create("C"), // [ A , C , B ]
            Operations::GoTo("A"),   // ["A", C , B ]
            Operations::Close,       // [   , C , B ]
            Operations::GoTo("B"),   // [   , C ,"B"]
            Operations::Close        // [   , C ,   ]
        ]
    )]
    #[case(
        &["A", "B"],
        &["A", "C"],
        &[
            Operations::GoTo("A"),   // [[A], B ,   ]
            Operations::Create("C"), // [ A , C , B ]
            Operations::GoTo("B"),   // [ A , C ,[B]]
            Operations::Close,       // [ A , C ,   ]
        ]
    )]
    #[case(
        &["A", "B"],
        &["B", "C"],
        &[
            Operations::GoTo("A"),  // ["A", B ,   ]
            Operations::Close,      // [   , B ,   ]
            Operations::GoTo("B"),  // [   ,"B",   ]
            Operations::Create("C") // [   , B , C ]
        ]
    )]
    #[case(
        &["A", "B", "C"],
        &["A", "B", "C", "D"],
        &[
            Operations::GoTo("C"),  // [ A , B ,"C",   ]
            Operations::Create("D") // [ A , B , C , D ]
        ]
    )]
    #[case(
        &["A", "B", "C"],
        &["A", "B", "D"],
        &[
            Operations::GoTo("B"),   // [ A ,"B",   , C ]
            Operations::Create("D"), // [ A , B , D , C ]
            Operations::GoTo("C"),   // [ A , B , D ,"C"]
            Operations::Close,       // [ A , B , D ,   ]
        ]
    )]
    #[case(
        &["A", "B", "C"],
        &["A", "D", "C"],
        &[
            Operations::GoTo("A"),   // ["A",   , B , C ]
            Operations::Create("D"), // [ A , D , B , C ]
            Operations::GoTo("B"),   // [ A , D ,"B", C ]
            Operations::Close,       // [ A , D ,   , C ]
        ]
    )]
    fn test_set_operations(
        #[case] current: &[&str],
        #[case] desired: &[&str],
        #[case] expected_operations: &[Operations<&str>],
    ) {
        let actual_operations = super::set_operations(
            current.iter().copied().collect(),
            desired.iter().copied().collect(),
        );
        assert_eq!(actual_operations, expected_operations);
    }
}
