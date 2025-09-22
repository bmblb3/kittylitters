use kittylitters::set_logic::{Operation, Solver};

#[rstest::rstest]
#[case(
    vec!["A"],
    vec!["A"],
    vec![]
)]
#[case(
    vec!["A", "B"],
    vec!["A", "B"],
    vec![]
)]
#[case(
    vec!["A", "B"],
    vec!["B", "A"],
    vec![
        Operation::GoTo(&"A"),
        Operation::MoveForward(1),
    ]
)]
#[case(
    vec!["A", "B"],
    vec!["A"],
    vec![
        Operation::GoTo(&"B"),
        Operation::Close
    ]
)]
#[case(
    vec!["A", "B"],
    vec!["B"],
    vec![
        Operation::GoTo(&"A"),
        Operation::Close
    ]
)]
#[case(
    vec!["A"],
    vec!["A", "B"],
    vec![
        Operation::GoTo(&"A"),
        Operation::New(&"B"),
    ]
)]
#[case(
    vec!["A", "B", "C"],
    vec!["A", "C"],
    vec![
        Operation::GoTo(&"B"),
        Operation::Close
    ]
)]
#[case(
    vec!["A"],
    vec!["B"],
    vec![
        Operation::GoTo(&"A"),
        Operation::New(&"B"),
        Operation::GoTo(&"A"),
        Operation::Close,
    ]
)]
#[case(
    vec!["A", "B", "C"],
    vec!["A", "B"],
    vec![
        Operation::GoTo(&"C"),
        Operation::Close,
    ]
)]
#[case(
    vec!["A", "B", "C"],
    vec!["A", "C", "B"],
    vec![
        Operation::GoTo(&"B"),     // vec!["A", [B], "C"]
        Operation::MoveForward(1), // vec!["A", "C", [B]]
    ]
)]
#[case(
    vec!["A", "B", "C"],
    vec!["B", "A", "C"],
    vec![
        Operation::GoTo(&"A"),     // vec![[A], "B", "C"]
        Operation::MoveForward(1), // vec!["B", [A], "C"]
    ]
)]
#[case(
    vec!["A", "B", "C"],
    vec!["B", "C", "A"],
    vec![
        Operation::GoTo(&"A"),     // vec![[A], "B", "C"]
        Operation::MoveForward(2), // vec!["B", [A], "C"]
                                   // vec!["B", "C", [A]]
    ]
)]
#[case(
    vec!["A", "B", "C"],
    vec!["C", "B", "A"],
    vec![
        Operation::GoTo(&"A"),     // vec![[A], "B", "C"]
        Operation::MoveForward(2), // vec!["B", [A], "C"]
                                   // vec!["B", "C", [A]]
        Operation::GoTo(&"B"),     // vec![[B], "C", "A"]
        Operation::MoveForward(1), // vec!["C", [B], "A"]
    ]
)]
#[case(
    vec!["A", "B", "C"],
    vec!["C", "A", "B"],
    vec![
        Operation::GoTo(&"A"),     // vec![[A], "B", "C"]
        Operation::MoveForward(1), // vec!["B", [A], "C"]
        Operation::GoTo(&"B"),     // vec![[B], "A", "C"]
        Operation::MoveForward(2), // vec!["A", [B], "C"]
                                   // vec!["A", "C", [B]]
        Operation::GoTo(&"A"),     // vec![[A], "C", "B"]
        Operation::MoveForward(1), // vec!["C", [A], "B"]

    ]
)]
#[case(
    vec!["A", "B"],
    vec!["C", "A"],
    vec![
        Operation::GoTo(&"A"),     // vec![[A], "B"]
        Operation::New(&"C"),      // vec!["A", [C], "B"]
        Operation::GoTo(&"A"),     // vec![[A], "C", "B"]
        Operation::MoveForward(1), // vec!["C", [A], "B"]
        Operation::GoTo(&"B"),     // vec!["C", "A", [B]]
        Operation::Close           // vec!["C", "A"]
    ]
)]
#[case(
    vec!["A", "B", "C"],
    vec!["D", "A", "B"],
    vec![
        Operation::GoTo(&"A"),     // vec![[A], "B", "C"]
        Operation::New(&"D"),      // vec!["A", [D], "B", "C"]
        Operation::GoTo(&"A"),     // vec![[A], "D", "B", "C"]
        Operation::MoveForward(1), // vec!["D", [A], "B", "C"]
        Operation::GoTo(&"C"),     // vec!["D", "A", "B", [C]]
        Operation::Close           // vec!["D", "A", "B"]
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
