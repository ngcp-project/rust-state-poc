// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use taurpc::Router;
use sqlx::postgres::PgPoolOptions;

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
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://ngcp:ngcp@localhost:5433/ngcpdb")
        .await
        .expect("Failed to connect to the database");
        
    let _cleanup_mission = sqlx::query("
    DROP TABLE IF EXISTS Mission CASCADE;
    ").execute(&pool).await.expect("Failed to execute query");
    
    let _cleanup_vehicle = sqlx::query("
    DROP TABLE IF EXISTS Vehicle CASCADE;
    ").execute(&pool).await.expect("Failed to execute query");
    
    let _cleanup_stage = sqlx::query("
    DROP TABLE IF EXISTS Stage CASCADE;
    ").execute(&pool).await.expect("Failed to execute query");
    
    let _cleanup_zone = sqlx::query("
    DROP TABLE IF EXISTS zones;
    ").execute(&pool).await.expect("Failed to execute query");


    let _create_mission_table = sqlx::query("
    CREATE TABLE IF NOT EXISTS Mission (
        missionName VARCHAR(255) PRIMARY KEY
    );
    ").execute(&pool).await.expect("Failed to execute query");


    let _create_vehicle_table = sqlx::query("
    CREATE TABLE IF NOT EXISTS Vehicle (
        vehicleName VARCHAR(255) PRIMARY KEY,
        missionName VARCHAR(255),
        currentStageID INT,
        FOREIGN KEY (missionName) REFERENCES Mission(missionName),
        CONSTRAINT missionVehicle UNIQUE (vehicleName, missionName)
    );
    ").execute(&pool).await.expect("Failed to execute query");

    let _create_stage_table = sqlx::query("
    CREATE TABLE IF NOT EXISTS Stage (
        stageID SERIAL PRIMARY KEY,
        stageName VARCHAR(255),
        vehicleName VARCHAR(255),
        FOREIGN KEY (vehicleName) REFERENCES Vehicle(vehicleName)
    );
    ").execute(&pool).await.expect("Failed to execute query");

    let _temp_zones = sqlx::query("
    CREATE TABLE IF NOT EXISTS zones (
        keepInZone TEXT,
        keepOutZone TEXT,
        searchArea TEXT,
        targetCoordinate TEXT,
        stageID INT PRIMARY KEY
    );
    ").execute(&pool).await.expect("Failed to execute query");


    let _vehicle_index = sqlx::query("
    CREATE INDEX idx_vehicle_currentStage
    ON Vehicle(currentStageID);
    ").execute(&pool).await;

    let _stage_index = sqlx::query("
    CREATE INDEX idx_stage_vehicle
    ON Stage(vehicleName);
    ").execute(&pool).await;

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(router.into_handler())
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}

