use std::sync::Mutex;
use tauri::{ window, Builder, Emitter, Manager, State, Window };

struct AppData {
  welcome_message: String,
  count: i32,
}
struct TelemetryData {
    
}

#[derive(Clone, serde::Serialize)]
struct MessagePayload {
  welcome_message: String,
}

#[derive(Clone, serde::Serialize)]
struct CountPayload {
  count: i32,
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str, app_data: State<Mutex<AppData>>, window: Window) -> String {
  let mut data = app_data.lock().unwrap();
  data.welcome_message = format!("Hello, {}! You've been greeted from Rust!", name);

  window
    .app_handle()
    .emit("welcome_message_changed", MessagePayload {
      welcome_message: data.welcome_message.clone(),
    })
    .unwrap();

  data.welcome_message.clone()
}

#[tauri::command]
fn get_welcome_message(app_data: State<Mutex<AppData>>) -> String {
  let data = app_data.lock().unwrap();
  data.welcome_message.clone()
}

#[tauri::command]
fn get_count(app_data: State<Mutex<AppData>>) -> i32 {
  let data = app_data.lock().unwrap();
  data.count
}

#[tauri::command]
fn set_count(count: i32, app_data: State<Mutex<AppData>>, window: Window) -> i32 {
  let mut data = app_data.lock().unwrap();
  data.count = count;
  print!("data.count {0}", data.count);
  window.app_handle().emit("count_changed", CountPayload { count }).unwrap();
  count
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  Builder::default()
    .setup(|app: &mut tauri::App| {
      app.manage(
        Mutex::new(AppData {
          welcome_message: String::from("Welcome to Tauri!"),
          count: 11,
        })
      );
      Ok(())
    })
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![greet, get_welcome_message, get_count, set_count])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
