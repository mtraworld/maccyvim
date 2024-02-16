use serde_json::Value as JsonValue;

use objc::{class, msg_send, sel, sel_impl};

#[cfg(target_os = "macos")]
pub fn check_accessibility_permissions() -> bool {
    let trusted = macos_accessibility_client::accessibility::application_is_trusted_with_prompt();
    if trusted {
        print!("Application is totally trusted!");
    } else {
        print!("Application isn't trusted :(");
    }
    trusted
}

pub fn generate_hints() -> Vec<String> {
    let mut hints = Vec::new();

    println!("Generating Hints...");

    // Retrieve actionable UI elements using macOS Accessibility APIs
    unsafe {
        let app: *mut objc::runtime::Object = msg_send![class!(NSApplication), sharedApplication];
        let window: *mut objc::runtime::Object = msg_send![app, keyWindow];
        let views: *mut objc::runtime::Object = msg_send![window, recursiveDescription];

        // Convert the views to a Rust string
        let views_str: *const ::std::os::raw::c_char = msg_send![views, UTF8String];
        let views_rust_str = std::ffi::CStr::from_ptr(views_str)
            .to_string_lossy()
            .into_owned();

        println!("Views as string: {}", views_rust_str);

        let views_json: serde_json::Value = serde_json::from_str(&views_rust_str).unwrap();
        println!("Views as JSON: {:?}", views_json);

        // Process the views to generate hints
        process_views(&views_json, &mut hints);
    }

    println!("Generated Hints: {:?}", hints);

    hints
}

fn process_views(view: &JsonValue, hints: &mut Vec<String>) {
    println!("Processing View: {:?}", view);
    // Process each view recursively
    if let Some(subviews) = view.get("subviews").and_then(|s| s.as_array()) {
        for subview in subviews {
            process_views(subview, hints);
        }
    }

    // Extract actionable UI elements and generate hints
    if let Some(class) = view.get("class").and_then(|s| s.as_str()) {
        println!("Class: {:?}", class);
        if let Some(frame) = view.get("frame").and_then(|f| f.as_str()) {
            println!("Frame: {:?}", frame);
            if let Some(action) = view.get("action").and_then(|a| a.as_str()) {
                println!("Action: {:?}", action);
                // Generate hint text based on the frame and action
                let hint_text = format!("Class: {}, Frame: {}, Action: {}", class, frame, action);
                hints.push(hint_text);
            }
        }
    }
}
