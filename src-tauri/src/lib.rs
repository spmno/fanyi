// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::thread;
use rdev::{listen, Event, EventType, Key};
use clippers::Clipboard;

static mut KEY_STORE:KeyStore;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_event_thread();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_event_thread() {
    thread::spawn(|| {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });
}

fn callback(event: Event) {
    match event.event_type{
        EventType::KeyPress(key) => {
            unsafe {
                KEY_STORE.current_key = key;
                match key {
                    Key::KeyC => {

                    },
                    Key::ControlLeft => {

                    }
                    _ => ()
                }
            }
            println!("press {:?}", key)
        },
        _ => ()
    }
}

struct KeyStore {
    current_key: Key,
    privious_key: Key,
}