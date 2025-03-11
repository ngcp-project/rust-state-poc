// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use taurpc::Router;

mod error;
mod stores;

use stores::counter::{CounterApiImpl, CounterApi};
use stores::form::api::{FormApiImpl, FormApi};
use stores::mission::api::{MissionApiImpl, MissionApi};
use stores::mission::api::{TestMissionAPI, MissionAPI};


fn setup_router() -> Router {
    // Initialize all the APIs here

    // use CounterApiImpl::new(initial_count) to set initial count
    // else use CounterApiImpl::default() to set initial count to 0
    let counter_api = CounterApiImpl::default(); 
    let form_api = FormApiImpl::default();
    let mission_api = TestMissionAPI::default();
    let mission_api = MissionApiImpl::default();
    
    Router::new()
        .merge(form_api.into_handler())
        .merge(counter_api.into_handler())
        .merge(mission_api.into_handler())
}


#[tokio::main]
async fn main() {
    let router = setup_router();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(router.into_handler())
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}

