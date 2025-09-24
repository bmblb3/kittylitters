use kittylitters::{
    set_logic::{Operation, Solver},
    windows::Window,
};

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
        Operation::GoTo("X/a"),
        Operation::MoveWindowForward(1),
    ]
)]
#[case(
    vec!["X/a", "X/b"],
    vec!["X/a"],
    vec![
        Operation::GoTo("X/b"),
        Operation::CloseWindow
    ]
)]
#[case(
    vec!["X/a", "X/b"],
    vec!["X/b"],
    vec![
        Operation::GoTo("X/a"),
        Operation::CloseWindow
    ]
)]
#[case(
    vec!["X/a"],
    vec!["X/a", "X/b"],
    vec![
        Operation::GoTo("X/a"),
        Operation::NewWindow("X/b"),
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/a", "X/c"],
    vec![
        Operation::GoTo("X/b"),
        Operation::CloseWindow
    ]
)]
#[case(
    vec!["X/a"],
    vec!["X/b"],
    vec![
        Operation::GoTo("X/a"),
        Operation::NewWindow("X/b"),
        Operation::GoTo("X/a"),
        Operation::CloseWindow,
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/a", "X/b"],
    vec![
        Operation::GoTo("X/c"),
        Operation::CloseWindow,
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/a", "X/c", "X/b"],
    vec![
        Operation::GoTo("X/b"),          // vec!["X/a", [X/b], "X/c"]
        Operation::MoveWindowForward(1), // vec!["X/a", "X/c", [X/b]]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/b", "X/a", "X/c"],
    vec![
        Operation::GoTo("X/a"),          // vec![[X/a], "X/b", "X/c"]
        Operation::MoveWindowForward(1), // vec!["X/b", [X/a], "X/c"]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/b", "X/c", "X/a"],
    vec![
        Operation::GoTo("X/a"),          // vec![[X/a], "X/b", "X/c"]
        Operation::MoveWindowForward(2), // vec!["X/b", [X/a], "X/c"]
                                         // vec!["X/b", "X/c", [X/a]]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/c", "X/b", "X/a"],
    vec![
        Operation::GoTo("X/a"),          // vec![[X/a], "X/b", "X/c"]
        Operation::MoveWindowForward(2), // vec!["X/b", [X/a], "X/c"]
                                         // vec!["X/b", "X/c", [X/a]]
        Operation::GoTo("X/b"),          // vec![[X/b], "X/c", "X/a"]
        Operation::MoveWindowForward(1), // vec!["X/c", [X/b], "X/a"]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/c", "X/a", "X/b"],
    vec![
        Operation::GoTo("X/a"),          // vec![[X/a], "X/b", "X/c"]
        Operation::MoveWindowForward(1), // vec!["X/b", [X/a], "X/c"]
        Operation::GoTo("X/b"),          // vec![[X/b], "X/a", "X/c"]
        Operation::MoveWindowForward(2), // vec!["X/a", [X/b], "X/c"]
                                         // vec!["X/a", "X/c", [X/b]]
        Operation::GoTo("X/a"),          // vec![[X/a], "X/c", "X/b"]
        Operation::MoveWindowForward(1), // vec!["X/c", [X/a], "X/b"]

    ]
)]
#[case(
    vec!["X/a", "X/b"],
    vec!["X/c", "X/a"],
    vec![
        Operation::GoTo("X/a"),          // vec![[X/a], "X/b"]
        Operation::NewWindow("X/c"),     // vec!["X/a", [X/c], "X/b"]
        Operation::GoTo("X/a"),          // vec![[X/a], "X/c", "X/b"]
        Operation::MoveWindowForward(1), // vec!["X/c", [X/a], "X/b"]
        Operation::GoTo("X/b"),          // vec!["X/c", "X/a", [X/b]]
        Operation::CloseWindow           // vec!["X/c", "X/a"]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/d", "X/a", "X/b"],
    vec![
        Operation::GoTo("X/a"),          // vec![[X/a], "X/b", "X/c"]
        Operation::NewWindow("X/d"),     // vec!["X/a", [X/d], "X/b", "X/c"]
        Operation::GoTo("X/a"),          // vec![[X/a], "X/d", "X/b", "X/c"]
        Operation::MoveWindowForward(1), // vec!["X/d", [X/a], "X/b", "X/c"]
        Operation::GoTo("X/c"),          // vec!["X/d", "X/a", "X/b", [X/c]]
        Operation::CloseWindow           // vec!["X/d", "X/a", "X/b"]
    ]
)]
fn test_cases(
    #[case] existing: Vec<&str>,
    #[case] desired: Vec<&str>,
    #[case] expected: Vec<Operation<&str>>,
) {
    let existing: Vec<Window> = existing
        .iter()
        .map(|title| Window::new(title.to_string()))
        .collect();
    let desired: Vec<Window> = desired
        .iter()
        .map(|title| Window::new(title.to_string()))
        .collect();

    let operations_on_windows = Solver::derive_operations(&existing, &desired);

    let mut operations: Vec<Operation<&str>> = Vec::new();
    for op in operations_on_windows {
        match op {
            Operation::GoTo(win) => operations.push(Operation::GoTo(&win.title)),
            Operation::MoveWindowForward(usize) => {
                operations.push(Operation::MoveWindowForward(usize))
            }
            Operation::CloseWindow => operations.push(Operation::CloseWindow),
            Operation::NewWindow(win) => operations.push(Operation::NewWindow(&win.title)),
        }
    }

    assert_eq!(expected, operations)
}
