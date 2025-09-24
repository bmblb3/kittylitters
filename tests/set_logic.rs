use kittylitters::set_logic::{Operation, Solver};

#[rstest::rstest]
#[case(
    vec!["X/a"],
    vec!["X/a"],
    vec![]
)]
#[case(
    vec!["X/a", "X/b"],
    vec!["X/a", "X/b"],
    vec![]
)]
#[case(
    vec!["X/a", "X/b"],
    vec!["X/b", "X/a"],
    vec![
        Operation::GoTo(&"X/a"),
        Operation::MoveForward(1),
    ]
)]
#[case(
    vec!["X/a", "X/b"],
    vec!["X/a"],
    vec![
        Operation::GoTo(&"X/b"),
        Operation::Close
    ]
)]
#[case(
    vec!["X/a", "X/b"],
    vec!["X/b"],
    vec![
        Operation::GoTo(&"X/a"),
        Operation::Close
    ]
)]
#[case(
    vec!["X/a"],
    vec!["X/a", "X/b"],
    vec![
        Operation::GoTo(&"X/a"),
        Operation::New(&"X/b"),
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/a", "X/c"],
    vec![
        Operation::GoTo(&"X/b"),
        Operation::Close
    ]
)]
#[case(
    vec!["X/a"],
    vec!["X/b"],
    vec![
        Operation::GoTo(&"X/a"),
        Operation::New(&"X/b"),
        Operation::GoTo(&"X/a"),
        Operation::Close,
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/a", "X/b"],
    vec![
        Operation::GoTo(&"X/c"),
        Operation::Close,
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/a", "X/c", "X/b"],
    vec![
        Operation::GoTo(&"X/b"),   // vec!["X/a", [X/b], "X/c"]
        Operation::MoveForward(1), // vec!["X/a", "X/c", [X/b]]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/b", "X/a", "X/c"],
    vec![
        Operation::GoTo(&"X/a"),   // vec![[X/a], "X/b", "X/c"]
        Operation::MoveForward(1), // vec!["X/b", [X/a], "X/c"]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/b", "X/c", "X/a"],
    vec![
        Operation::GoTo(&"X/a"),   // vec![[X/a], "X/b", "X/c"]
        Operation::MoveForward(2), // vec!["X/b", [X/a], "X/c"]
                                   // vec!["X/b", "X/c", [X/a]]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/c", "X/b", "X/a"],
    vec![
        Operation::GoTo(&"X/a"),   // vec![[X/a], "X/b", "X/c"]
        Operation::MoveForward(2), // vec!["X/b", [X/a], "X/c"]
                                   // vec!["X/b", "X/c", [X/a]]
        Operation::GoTo(&"X/b"),   // vec![[X/b], "X/c", "X/a"]
        Operation::MoveForward(1), // vec!["X/c", [X/b], "X/a"]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/c", "X/a", "X/b"],
    vec![
        Operation::GoTo(&"X/a"),   // vec![[X/a], "X/b", "X/c"]
        Operation::MoveForward(1), // vec!["X/b", [X/a], "X/c"]
        Operation::GoTo(&"X/b"),   // vec![[X/b], "X/a", "X/c"]
        Operation::MoveForward(2), // vec!["X/a", [X/b], "X/c"]
                                   // vec!["X/a", "X/c", [X/b]]
        Operation::GoTo(&"X/a"),   // vec![[X/a], "X/c", "X/b"]
        Operation::MoveForward(1), // vec!["X/c", [X/a], "X/b"]

    ]
)]
#[case(
    vec!["X/a", "X/b"],
    vec!["X/c", "X/a"],
    vec![
        Operation::GoTo(&"X/a"),   // vec![[X/a], "X/b"]
        Operation::New(&"X/c"),    // vec!["X/a", [X/c], "X/b"]
        Operation::GoTo(&"X/a"),   // vec![[X/a], "X/c", "X/b"]
        Operation::MoveForward(1), // vec!["X/c", [X/a], "X/b"]
        Operation::GoTo(&"X/b"),   // vec!["X/c", "X/a", [X/b]]
        Operation::Close           // vec!["X/c", "X/a"]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/d", "X/a", "X/b"],
    vec![
        Operation::GoTo(&"X/a"),   // vec![[X/a], "X/b", "X/c"]
        Operation::New(&"X/d"),    // vec!["X/a", [X/d], "X/b", "X/c"]
        Operation::GoTo(&"X/a"),   // vec![[X/a], "X/d", "X/b", "X/c"]
        Operation::MoveForward(1), // vec!["X/d", [X/a], "X/b", "X/c"]
        Operation::GoTo(&"X/c"),   // vec!["X/d", "X/a", "X/b", [X/c]]
        Operation::Close           // vec!["X/d", "X/a", "X/b"]
    ]
)]
fn test_cases(
    #[case] existing: Vec<&str>,
    #[case] desired: Vec<&str>,
    #[case] expected: Vec<Operation<&&str>>,
) {
    let actual = Solver::derive_operations(&existing, &desired);
    assert_eq!(expected, actual)
}
