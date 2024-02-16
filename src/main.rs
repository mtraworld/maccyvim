mod accessibility;
mod hint_mode;
mod keyboard_handler;
mod scroll_mode;

use accessibility::generate_hints;
use keyboard_handler::start_keyboard_listener;

fn main() {
    // Start the keyboard listener
    start_keyboard_listener();

    // Generate hints for actionable UI elements
    let hints = generate_hints();

    // Print the generated hints
    println!("Generated Hints:");
    for hint in hints {
        println!("{}", hint);
    }
}
