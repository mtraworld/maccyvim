use crate::{hint_mode::activate_hint_mode, scroll_mode::activate_scroll_mode};
use global_hotkey::GlobalHotKeyEvent;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager, HotKeyState,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tao::event_loop::{ControlFlow, EventLoopBuilder};

pub fn start_keyboard_listener() {
    let hint_mode_active = Arc::new(AtomicBool::new(false));
    let scroll_mode_active = Arc::new(AtomicBool::new(false));

    let hint_mode_clone = hint_mode_active.clone();
    let _scroll_mode_clone = scroll_mode_active.clone();

    let event_loop = EventLoopBuilder::new().build();

    let manager = GlobalHotKeyManager::new().unwrap();

    // Construct the hotkey for activating Hint-mode (e.g., control + Space key)
    let hint_mode_hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::Space);

    // Register the hotkey
    manager.register(hint_mode_hotkey).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if hint_mode_active.load(Ordering::Relaxed) {
            // Activate Hint-mode
            activate_hint_mode();
            hint_mode_active.store(false, Ordering::Relaxed);
        }

        if scroll_mode_active.load(Ordering::Relaxed) {
            // Activate Scroll-mode
            activate_scroll_mode();
            scroll_mode_active.store(false, Ordering::Relaxed);
        }

        if let Ok(event) = global_hotkey_channel.try_recv() {
            println!("{event:?}");
            if hint_mode_hotkey.id() == event.id && event.state == HotKeyState::Released {
                hint_mode_clone.store(true, Ordering::Relaxed);
                // hint_mode_active.store(true, Ordering::Relaxed);
                activate_hint_mode();
            }
        }
    });
}
