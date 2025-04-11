// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::Result;

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
    DROP TABLE IF EXISTS missions CASCADE;
    ").execute(&pool).await.expect("Failed to execute query");
    
    let _cleanup_vehicle = sqlx::query("
    DROP TABLE IF EXISTS vehicles CASCADE;
    ").execute(&pool).await.expect("Failed to execute query");
    
    let _cleanup_stage = sqlx::query("
    DROP TABLE IF EXISTS stages CASCADE;
    ").execute(&pool).await.expect("Failed to execute query");

    // This is to create the status enum but I dont think it's needed to be executed more than once (hence why this is commented out)
    //
    // let _create_status_type = sqlx::query("
    // DO $$
    // BEGIN
    //     IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'status') THEN
    //         CREATE TYPE status AS ENUM ('Active', 'Inactive', 'Complete', 'Failed');
    //     END IF;
    // END $$;
    // ").execute(&pool).await.expect("Failed to create type 'status'");

    let _create_mission_table = sqlx::query("
    CREATE TABLE IF NOT EXISTS missions (
        mission_name VARCHAR(255) PRIMARY KEY,
        keep_in_zones TEXT[] NOT NULL,
        keep_out_zones TEXT[] NOT NULL,
        status status
    );
    ").execute(&pool).await.expect("Failed to create table 'missions'");


    let _create_vehicle_table = sqlx::query("
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
    ").execute(&pool).await.expect("Failed to execute query");

    let _add_unique_constraint = sqlx::query("
    ALTER TABLE vehicles ADD CONSTRAINT vehicle_name_unique UNIQUE (vehicle_name);
    ").execute(&pool).await.expect("Failed to add unique constraint");

    let _create_stage_table = sqlx::query("
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
    ").execute(&pool).await.expect("Failed to execute query");

    let _vehicle_index = sqlx::query("
    CREATE INDEX idx_vehicle_currentStage
    ON Vehicle(currentStageID);
    ").execute(&pool).await;

    let _stage_index = sqlx::query("
    CREATE INDEX idx_stage_vehicle
    ON Stage(vehicleName);
    ").execute(&pool).await;

    // join to get all missions
    let missions = sqlx::query(
        "
        SELECT 
            missions.mission_name,
            missions.status AS mission_status,
            missions.keep_in_zones,
            missions.keep_out_zones,
            vehicles.vehicle_name,
            vehicles.current_stage_id AS current_stage,
            vehicles.is_auto,
            vehicles.patient_status,
            stages.stage_id,
            stages.stage_name,
            stages.search_area,
            stages.target_coordinate
        FROM missions
        LEFT JOIN vehicles ON vehicles.mission_name = missions.mission_name
        LEFT JOIN stages 
            ON stages.vehicle_name = vehicles.vehicle_name
            AND stages.mission_name = vehicles.mission_name;
        "
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to execute query");

    // Print the results
    println!("Results: {:?}", missions);
    // println("TEST AWAWAWAWAWAWAWAWA");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(router.into_handler())
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}

