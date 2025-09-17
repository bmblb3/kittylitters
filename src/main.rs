use kittylitters::collect_windows::collect_windows;
use kittylitters::get_focused_window_id::get_focused_window_id;
use kittylitters::read_yml::read_sess_yml;

fn main() {
    let colleted_windows = collect_windows().unwrap();
    println!("Existing windows: {:?}", colleted_windows.0);

    let init_focused_window_id = get_focused_window_id().unwrap();
    println!("\nFocused window ID: {}", init_focused_window_id);

    if !colleted_windows.verify() {
        panic!("\nExisiting tab-window names are not unique!")
    };

    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let config_path = format!("{}/.config/kitty/session.yml", home_dir);
    let desired_tabs = read_sess_yml(&config_path).expect("Failed to read config.yml");
    println!("\nDesired tabs: {:?}", desired_tabs.0);

    let to_add = desired_tabs.sub(&colleted_windows);
    println!("\nTo add: {:?}", to_add.0);

    let to_remove = colleted_windows.sub(&desired_tabs);
    println!("\nTo remove: {:?}", to_remove.0);
}
