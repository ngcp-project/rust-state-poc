// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use taurpc::Router;
use sqlx::postgres::PgConnection;
use sqlx::Connection;

use sqlx::query;
mod error;
mod stores;

use stores::counter::{CounterApiImpl, CounterApi};
use stores::form::api::{FormApiImpl, FormApi};
use stores::mission::api::{MissionApiImpl, MissionApi};


async fn setup_router() -> Router {
    // Initialize all the APIs here

    // use CounterApiImpl::new(initial_count) to set initial count
    // else use CounterApiImpl::default() to set initial count to 0
    let counter_api = CounterApiImpl::default(); 
    let form_api = FormApiImpl::default();
    let mission_api = MissionApiImpl::new().await;
    
    Router::new()
        .merge(form_api.into_handler())
        .merge(counter_api.into_handler())
        .merge(mission_api.into_handler())
}

async fn initialize_database() {
    let mut db_conn = PgConnection::connect("postgres://ngcp:ngcp@localhost:5433/ngcpdb").await.expect("Failed to connect to the database");

    let _cleanup_mission = query("
    DROP TABLE IF EXISTS missions CASCADE;
    ").execute(&mut db_conn).await.expect("Failed to execute query");
    
    let _cleanup_vehicle = query("
    DROP TABLE IF EXISTS vehicles CASCADE;
    ").execute(&mut db_conn).await.expect("Failed to execute query");
    
    let _cleanup_stage = query("
    DROP TABLE IF EXISTS stages CASCADE;
    ").execute(&mut db_conn).await.expect("Failed to execute query");

    let _cleanup_test = query("
    DROP TABLE IF EXISTS test;
    ").execute(&mut db_conn).await.expect("Failed to execute query");

    let _create_test_table = query("
    CREATE TABLE IF NOT EXISTS test (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        age INTEGER NOT NULL
    );
    ").execute(&mut db_conn).await.expect("Failed to create table 'test'");


    let _create_status_type = query("
    DO $$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'status') THEN
            CREATE TYPE status AS ENUM ('Active', 'Inactive', 'Complete', 'Failed');
        END IF;
    END $$;
    ").execute(&mut db_conn).await.expect("Failed to create type 'status'");

    let _create_mission_table = query("
    CREATE TABLE IF NOT EXISTS missions (
        mission_name VARCHAR(255) PRIMARY KEY,
        keep_in_zones TEXT[] NOT NULL,
        keep_out_zones TEXT[] NOT NULL,
        status status
    );
    ").execute(&mut db_conn).await.expect("Failed to create table 'missions'");


    let _create_vehicle_table = query("
    CREATE TABLE IF NOT EXISTS vehicles (
        mission_name VARCHAR(255) NOT NULL,
        vehicle_name VARCHAR(255) NOT NULL,
        current_stage_id INTEGER NOT NULL,
        PRIMARY KEY (mission_name, vehicle_name),
        CONSTRAINT fk_mission_name
        FOREIGN KEY (mission_name)
        REFERENCES missions (mission_name)
            ON DELETE CASCADE
            ON UPDATE CASCADE
    );
    ").execute(&mut db_conn).await.expect("Failed to execute query");

    let _add_unique_constraint = query("
    ALTER TABLE vehicles ADD CONSTRAINT vehicle_name_unique UNIQUE (vehicle_name);
    ").execute(&mut db_conn).await.expect("Failed to add unique constraint");

    let _create_stage_table = query("
    CREATE TABLE IF NOT EXISTS stages (
        stage_id SERIAL PRIMARY KEY,
        vehicle_name VARCHAR(255) NOT NULL,
        search_area TEXT[] NOT NULL,       
        stage_name VARCHAR(255) NOT NULL,
        target_coordinate TEXT NOT NULL, 
        CONSTRAINT fk_vehicle_name
        FOREIGN KEY (vehicle_name)
        REFERENCES vehicles (vehicle_name)
    );
    ").execute(&mut db_conn).await.expect("Failed to execute query");

    let _vehicle_index = query("
    CREATE INDEX idx_vehicle_currentStage
    ON Vehicle(currentStageID);
    ").execute(&mut db_conn).await;

    let _stage_index = query("
    CREATE INDEX idx_stage_vehicle
    ON Stage(vehicleName);
    ").execute(&mut db_conn).await;

    db_conn.close().await.expect("Failed to close database connection");
}


#[tokio::main]
async fn main() {    
    initialize_database().await;

    let router = setup_router();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(router.await.into_handler())
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}

