mod accessibility;
mod hint_mode;
mod keyboard_handler;
mod scroll_mode;

use keyboard_handler::start_keyboard_listener;

fn main() {
    // Check if the application has accessibility permissions
    if !accessibility::check_accessibility_permissions() {
        println!("Accessibility permissions not granted. Requesting permissions...");
    } else {
        println!("Accessibility permissions granted.");
    }

    // Start the keyboard listener
    start_keyboard_listener();
}
