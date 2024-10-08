// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::thread;
use rdev::{listen, Event, EventType, Key};
use clippers::Clipboard;
use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Emitter, Manager};
use std::sync::OnceLock;

static GLOBAL_APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

static mut KEY_STORE:KeyStore = KeyStore{
    current_key : Key::KeyA,
    privious_key : Key::KeyA
};


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_event_thread();
    tauri::Builder::default()
        .setup(|app|{
            #[cfg(debug_assertions)] // 仅在调试构建时包含此代码
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                //window.close_devtools();
            }
            GLOBAL_APP_HANDLE.set(app.handle().clone()).expect("Failed to set global app handle");
            Ok(())
        })
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

#[derive(Serialize)]
enum TargetLang {
    #[serde(rename(serialize = "ZH-HANS"))]
    ZHHANS,
    EN
}

#[derive(Serialize)]
struct DeepLRequest {
    text: Vec<String>,
    target_lang: TargetLang
}

#[derive(Deserialize)]
struct DeepLResponse {
    translations: Vec<TranslateResult>
}

#[derive(Deserialize)]
struct TranslateResult {
    detected_source_language: String,
    text: String
}

fn translate() {
    println!("translate.");
    let mut clipboard = Clipboard::get();
    match clipboard.read() {
        Some(clippers::ClipperData::Text(text)) => {
            println!("Clipboard text: {:?}", text);
            let client = reqwest::blocking::Client::new();
            let url = "https://api-free.deepl.com/v2/translate";
            let auth_key = std::env::var("DEEPL_API_KEY").unwrap();
            let token = "DeepL-Auth-Key ".to_string() + &auth_key;
            let req: DeepLRequest = DeepLRequest {
                target_lang : TargetLang::ZHHANS,
                text: vec![text.to_string()]
            };
            let res = client.post(url)
                .header(reqwest::header::AUTHORIZATION, token)
                .json(&req)
                .send();
            match res {
                Ok(res) => {
                    //println!("res: {}", res.text().unwrap());
                    let result = res.json::<DeepLResponse>().unwrap();
                    send_text(GLOBAL_APP_HANDLE.get().unwrap().clone(), result.translations[0].text.clone());
                    println!("result: {}", result.translations[0].text);
                },
                Err(err) => {
                    println!("err: {}", err);
                }
            }
        }

        Some(clippers::ClipperData::Image(image)) => {
            println!("Clipboard image: {}x{} RGBA", image.width(), image.height());
        }

        Some(data) => {
            println!("Clipboard data is unknown: {data:?}");
        }

        None => {
            send_text(GLOBAL_APP_HANDLE.get().unwrap().clone(), "".to_string());
            println!("Clipboard is empty");
        }
    }
}

fn callback(event: Event) {
    match event.event_type{
        EventType::KeyPress(key) => {
            unsafe {
                KEY_STORE.current_key = key;
                match key {
                    Key::KeyC => {
                        if KEY_STORE.privious_key == Key::ControlLeft {
                            start_fanyi(GLOBAL_APP_HANDLE.get().unwrap().clone());
                            translate();
                            KEY_STORE.clear_status();
                        }
                    },
                    Key::ControlLeft => {
                        if KEY_STORE.privious_key == Key::KeyC {
                            translate();
                            KEY_STORE.clear_status();
                        }
                    }
                    _ => ()
                }
                KEY_STORE.privious_key = KEY_STORE.current_key;

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

impl KeyStore {
    fn clear_status(&mut self) {
        self.privious_key = Key::KeyA;
        self.current_key = Key::KeyA;
    }
}

#[tauri::command]
fn send_text(app: AppHandle, text: String) {
    //let webview = app.get_webview_window("main").unwrap();
    //webview.eval("console.log('hello from Rust')").unwrap();
    //app.emit_to("fanyi", "main", text).unwrap();
    app.emit_to("main", "fanyi", text).unwrap();
}

#[tauri::command]
fn start_fanyi(app: AppHandle) {
    //let webview = app.get_webview_window("main").unwrap();
    //webview.eval("console.log('hello from Rust')").unwrap();
    //app.emit_to("fanyi", "main", text).unwrap();
    app.emit_to("main", "start", "").unwrap();
}