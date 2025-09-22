use kittylitters::collect_windows::collect_windows;
use kittylitters::read_yml::read_session_yml;

fn main() {
    let current_windows = collect_windows().unwrap();
    println!("Existing windows: {:?}", current_windows);

    let home_dir = std::env::var("HOME").expect("HOME environment variable should be set");
    let config_path = format!("{}/.config/kitty/session.yml", home_dir);
    let desired_windows = read_session_yml(&config_path)
        .expect("Expect a valid yml file at ~/.config/kitty/session.yml");
    println!("\nDesired windows: {:?}", desired_windows);
}
