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
    loop {
        if desired_set.iter().eq(&current_set) {
            break;
        }

        let cs = current_set.clone();
        let mut current_iter = cs.iter();
        let mut desired_iter = desired_set.iter();
        let mut latest_aligned = None;

        loop {
            let this_item = current_iter.next().cloned();
            let target_item = desired_iter.next().cloned();
            let mut this_operations = Vec::new();

            if this_item.is_none() && target_item.is_none() {
                break;
            }

            if let Some(this_item) = this_item
                && let Some(target_item) = &target_item
                && &this_item == target_item
            {
                latest_aligned = Some(this_item);
                continue;
            }

            if let Some(target_item) = &target_item
                && !&current_set.contains(target_item)
            {
                let goto_item = if let Some(latest_aligned) = latest_aligned {
                    latest_aligned
                } else {
                    *current_set
                        .first()
                        .expect("`current_set` should have a first element")
                };
                this_operations.push(Operations::GoTo(goto_item));
                let this_index = current_set.get_index_of(&goto_item).expect(
                    "`goto_item` should be present in `current_set` since it
                 was picked from there",
                );

                this_operations.push(Operations::Create(*target_item));
                current_set.shift_insert(this_index + 1, *target_item);
            }

            if let Some(this_item) = &this_item
                && !&desired_set.contains(this_item)
            {
                this_operations.push(Operations::GoTo(*this_item));
                let this_index = current_set.get_index_of(this_item).expect(
                    "`this_item` should be present in `current_set` since it was picked from there",
                );

                this_operations.push(Operations::Close);
                current_set.shift_remove_index(this_index);
            }

            for op in this_operations {
                operations.push(op);
            }
        }
    }
    operations
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
    #[case(
        &["A", "B", "C"],
        &["A", "D", "B"],
        &[
            Operations::GoTo("A"),   // ["A",   , B , C ]
            Operations::Create("D"), // [ A , D , B , C ]
            Operations::GoTo("C"),   // [ A , D , B ,"C"]
            Operations::Close,       // [ A , D , B ,   ]
        ]
    )]
    #[case(
        &["A", "B", "C"],
        &["D", "B", "C"],
        &[
            Operations::GoTo("A"),   // ["A",   , B , C ]
            Operations::Create("D"), // [ A , D , B , C ]
            Operations::GoTo("A"),   // ["A", D , B ,"C"]
            Operations::Close,       // [   , D , B , C ]
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
