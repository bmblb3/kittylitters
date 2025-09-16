use kittylitters::get_focused_window_id;

fn main() {
    let init_focused_window_id = get_focused_window_id::get_focused_window_id().unwrap();
    println!("{}", init_focused_window_id);
}
