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
        Operation::MoveWindowForward,
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
        Operation::GoTo("X/b"),       // vec!["X/a", [X/b], "X/c"]
        Operation::MoveWindowForward, // vec!["X/a", "X/c", [X/b]]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/b", "X/a", "X/c"],
    vec![
        Operation::GoTo("X/a"),       // vec![[X/a], "X/b", "X/c"]
        Operation::MoveWindowForward, // vec!["X/b", [X/a], "X/c"]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/b", "X/c", "X/a"],
    vec![
        Operation::GoTo("X/a"),       // vec![[X/a], "X/b", "X/c"]
        Operation::MoveWindowForward, // vec!["X/b", [X/a], "X/c"]
        Operation::MoveWindowForward, // vec!["X/b", "X/c", [X/a]]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/c", "X/b", "X/a"],
    vec![
        Operation::GoTo("X/a"),       // vec![[X/a], "X/b", "X/c"]
        Operation::MoveWindowForward, // vec!["X/b", [X/a], "X/c"]
        Operation::MoveWindowForward, // vec!["X/b", "X/c", [X/a]]
        Operation::GoTo("X/b"),       // vec![[X/b], "X/c", "X/a"]
        Operation::MoveWindowForward, // vec!["X/c", [X/b], "X/a"]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/c", "X/a", "X/b"],
    vec![
        Operation::GoTo("X/a"),       // vec![[X/a], "X/b", "X/c"]
        Operation::MoveWindowForward, // vec!["X/b", [X/a], "X/c"]
        Operation::GoTo("X/b"),       // vec![[X/b], "X/a", "X/c"]
        Operation::MoveWindowForward, // vec!["X/a", [X/b], "X/c"]
        Operation::MoveWindowForward, // vec!["X/a", "X/c", [X/b]]
        Operation::GoTo("X/a"),       // vec![[X/a], "X/c", "X/b"]
        Operation::MoveWindowForward, // vec!["X/c", [X/a], "X/b"]

    ]
)]
#[case(
    vec!["X/a", "X/b"],
    vec!["X/c", "X/a"],
    vec![
        Operation::GoTo("X/a"),       // vec![[X/a], "X/b"]
        Operation::NewWindow("X/c"),  // vec!["X/a", [X/c], "X/b"]
        Operation::GoTo("X/a"),       // vec![[X/a], "X/c", "X/b"]
        Operation::MoveWindowForward, // vec!["X/c", [X/a], "X/b"]
        Operation::GoTo("X/b"),       // vec!["X/c", "X/a", [X/b]]
        Operation::CloseWindow        // vec!["X/c", "X/a"]
    ]
)]
#[case(
    vec!["X/a", "X/b", "X/c"],
    vec!["X/d", "X/a", "X/b"],
    vec![
        Operation::GoTo("X/a"),       // vec![[X/a], "X/b", "X/c"]
        Operation::NewWindow("X/d"),  // vec!["X/a", [X/d], "X/b", "X/c"]
        Operation::GoTo("X/a"),       // vec![[X/a], "X/d", "X/b", "X/c"]
        Operation::MoveWindowForward, // vec!["X/d", [X/a], "X/b", "X/c"]
        Operation::GoTo("X/c"),       // vec!["X/d", "X/a", "X/b", [X/c]]
        Operation::CloseWindow        // vec!["X/d", "X/a", "X/b"]
    ]
)]
#[case(
    vec!["X/a"],
    vec!["X/a", "Y/a"],
    vec![
        Operation::GoTo("X/a"),       // vec![[X/a]]
        Operation::NewTab("Y/a"),     // vec!["X/a", "Y/a"]
    ]
)]
#[case(
    vec!["X/a", "Y/a"],
    vec!["Y/a", "X/a"],
    vec![
        Operation::GoTo("X/a"),       // vec![[X/a], "Y/a"]
        Operation::MoveTabForward,    // vec!["Y/a", [X/a]]
    ]
)]
#[case(
    vec!["X/a", "X/b", "Y/a"],
    vec!["Y/a", "X/a", "X/b"],
    vec![
        Operation::GoTo("X/a"),       // vec![[X/a], "X/b", "Y/a"]
        Operation::MoveTabForward,    // vec!["Y/a", [X/a], "X/b"]
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

    let actual = Solver::derive_operations(&existing, &desired);
    let actual: Vec<Operation<String>> = actual
        .into_iter()
        .map(|op| match op {
            Operation::GoTo(w) => Operation::GoTo(w.title),
            Operation::MoveTabForward => Operation::MoveTabForward,
            Operation::MoveWindowForward => Operation::MoveWindowForward,
            Operation::CloseWindow => Operation::CloseWindow,
            Operation::NewTab(w) => Operation::NewTab(w.title),
            Operation::NewWindow(w) => Operation::NewWindow(w.title),
        })
        .collect();

    let expected: Vec<Operation<String>> = expected
        .into_iter()
        .map(|op| match op {
            Operation::GoTo(s) => Operation::GoTo(s.to_string()),
            Operation::MoveTabForward => Operation::MoveTabForward,
            Operation::MoveWindowForward => Operation::MoveWindowForward,
            Operation::CloseWindow => Operation::CloseWindow,
            Operation::NewTab(s) => Operation::NewTab(s.to_string()),
            Operation::NewWindow(s) => Operation::NewWindow(s.to_string()),
        })
        .collect();

    assert_eq!(expected, actual)
}
