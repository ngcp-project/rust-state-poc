// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sea_orm::{Database, DatabaseConnection, DbErr};
use taurpc::Router;

mod error;
mod stores;

use stores::counter::{CounterApiImpl, CounterApi};
use stores::form::api::{FormApiImpl, FormApi};
use stores::mission::api::{MissionApiImpl, MissionApi};


fn setup_router() -> Router {
    // Initialize all the APIs here

    // use CounterApiImpl::new(initial_count) to set initial count
    // else use CounterApiImpl::default() to set initial count to 0
    let counter_api = CounterApiImpl::default(); 
    let form_api = FormApiImpl::default();
    let mission_api = MissionApiImpl::default();
    
    Router::new()
        .merge(form_api.into_handler())
        .merge(counter_api.into_handler())
        .merge(mission_api.into_handler())
}


#[tokio::main]
async fn main() {
    let router = setup_router();
    let db: DatabaseConnection = Database::connect("postgres://ngcp:ngcp@localhost:5433/ngcpdb")
        .await
        .expect("Failed to connect to the database");

    assert!(db.ping().await.is_ok());
    db.clone().close().await.expect("Failed to close the database connection");
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire(_))));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(router.into_handler())
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}

