use kittylitters::collect_windows::collect_windows;
use kittylitters::get_focused_window_id::get_focused_window_id;

fn main() {
    let init_focused_window_id = get_focused_window_id().unwrap();
    let colleted_windows = collect_windows().unwrap();
    if !colleted_windows.verify() {
        panic!("Exisiting tab-window names are not unique!")
    };
}
