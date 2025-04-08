// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use taurpc::Router;
use sqlx::postgres::PgConnection;
use sqlx::Connection;

use sqlx::{query, Row};
mod error;
mod stores;

use stores::counter::{CounterApiImpl, CounterApi};
use stores::form::api::{FormApiImpl, FormApi};
use stores::mission::api::{MissionApiImpl, MissionApi};

const DB_URL: &str = "postgres://ngcp:ngcp@localhost:5433/ngcpdb";


async fn init_database_dummy_data() {
    let mut db_conn = PgConnection::connect(DB_URL).await.expect("Failed to connect to the database");

    let _insert_dummy_discover_mission = query("
        INSERT INTO missions(mission_name, keep_in_zones, keep_out_zones, status) 
        VALUES ($1, $2, $3, $4::status)
    ")
    .bind("Discover Mission")
    .bind(&vec![
        r#"[
            {"latitude":37.33285,"longitude":-122.34302},
            {"latitude":51.54564,"longitude":-0.49298},
            {"latitude":-33.78501,"longitude":151.29494},
            {"latitude":40.12456,"longitude":-74.72894},
            {"latitude":56.94295,"longitude":3.97837}
        ]"#.to_string(),
        r#"[
            {"latitude":48.33285,"longitude":-73.34302},
            {"latitude":-12.54564,"longitude":103.49298},
            {"latitude":21.78501,"longitude":-88.29494},
            {"latitude":59.12456,"longitude":12.72894},
            {"latitude":-4.94295,"longitude":145.97837}
        ]"#.to_string()
    ])
    .bind(&vec![
        r#"[
            {"latitude":-41.23756,"longitude":38.29417},
            {"latitude":62.23701,"longitude":-104.23486},
            {"latitude":-16.98743,"longitude":113.93240},
            {"latitude":49.89453,"longitude":-9.89456},
            {"latitude":-33.12789,"longitude":72.24690}
        ]"#.to_string(),
        r#"[
            {"latitude":28.23847,"longitude":102.35892},
            {"latitude":-12.98237,"longitude":-44.23510},
            {"latitude":45.23456,"longitude":8.65412},
            {"latitude":-39.76892,"longitude":58.71245},
            {"latitude":23.43258,"longitude":-82.35821}
        ]"#.to_string()
    ])
    .bind("Inactive")
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into missions");


    let _insert_dummy_retrieve_mission = query("
        INSERT INTO missions(mission_name, keep_in_zones, keep_out_zones, status) 
        VALUES ($1, $2, $3, $4::status)
    ") // Assuming mission_id is 1 for the first mission
    .bind("Retrieve Mission")
    .bind(&vec![
        r#"[
            {"latitude":5.23657,"longitude":-68.74629},
            {"latitude":33.54321,"longitude":-101.59834},
            {"latitude":-28.23471,"longitude":85.94732},
            {"latitude":12.59481,"longitude":77.24362},
            {"latitude":-53.78192,"longitude":124.87453}
        ]"#.to_string(),
        r#"[
            {"latitude":49.23849,"longitude":-87.15234},
            {"latitude":-13.78657,"longitude":-102.43578},
            {"latitude":61.18436,"longitude":17.94861},
            {"latitude":21.38940,"longitude":-13.24867},
            {"latitude":-45.89267,"longitude":122.73901}
        ]"#.to_string()
    ])
    .bind(&vec![
        r#"[
            {"latitude":34.54319,"longitude":101.63489},
            {"latitude":-5.89234,"longitude":56.23418},
            {"latitude":28.95762,"longitude":-115.72139},
            {"latitude":-50.34217,"longitude":32.94123},
            {"latitude":13.98312,"longitude":-79.87655}
        ]"#.to_string(),
        r#"[
            {"latitude":-26.19243,"longitude":110.73284},
            {"latitude":62.98123,"longitude":-43.89357},
            {"latitude":-35.78420,"longitude":99.28964},
            {"latitude":22.84656,"longitude":-68.12345},
            {"latitude":48.23950,"longitude":79.56439}
        ]"#.to_string()
    ])
    .bind("Inactive")
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into missions");


    

    // let test_result = query("
    //     SELECT * FROM missions WHERE mission_name = $1
    // ")
    // .bind("Discover Mission")
    // .fetch_all(&mut db_conn)
    // .await
    // .expect("Failed to fetch dummy data from missions");

    
    // for row in test_result {
    //     let mission_id: i32 = row.get("mission_id");
    //     let mission_name: String = row.get("mission_name");
    //     let keep_in_zones: Vec<String> = row.get("keep_in_zones");
    //     let keep_out_zones: Vec<String> = row.get("keep_out_zones");
    //     println!("Mission ID: {}, Name: {}, Keep In Zones: {:?}, Keep Out Zones: {:?}", 
    //         mission_id, mission_name, keep_in_zones, keep_out_zones);
    // }
    



    db_conn.close().await.expect("Failed to close database connection");
}



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
    let mut db_conn = PgConnection::connect(DB_URL).await.expect("Failed to connect to the database");

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
        mission_id SERIAL PRIMARY KEY,
        mission_name VARCHAR(255),
        keep_in_zones TEXT[] NOT NULL,
        keep_out_zones TEXT[] NOT NULL,
        status status
    );
    ").execute(&mut db_conn).await.expect("Failed to create table 'missions'");


    let _create_vehicle_table = query("
    CREATE TABLE IF NOT EXISTS vehicles (
        mission_id INTEGER REFERENCES missions ON DELETE CASCADE,
        vehicle_id SERIAL UNIQUE,
        vehicle_name VARCHAR(255) NOT NULL,
        current_stage_id INTEGER NOT NULL,
        PRIMARY KEY (mission_id, vehicle_id)
    );
    ").execute(&mut db_conn).await.expect("Failed to execute query");

    let _create_stage_table = query("
    CREATE TABLE IF NOT EXISTS stages (
        stage_id SERIAL PRIMARY KEY,
        vehicle_id INTEGER REFERENCES vehicles(vehicle_id) ON DELETE CASCADE,
        search_area TEXT[] NOT NULL,      
        stage_name VARCHAR(255) NOT NULL,
        target_coordinate TEXT NOT NULL
    );
    ").execute(&mut db_conn).await.expect("Failed to execute query");


    // NOTE: Not sure if these indexes are needed, but keeping them for now
    // let _vehicle_index = query("
    // CREATE INDEX idx_vehicle_currentStage
    // ON Vehicle(currentStageID);
    // ").execute(&mut db_conn).await;

    // let _stage_index = query("
    // CREATE INDEX idx_stage_vehicle
    // ON Stage(vehicleName);
    // ").execute(&mut db_conn).await;

    db_conn.close().await.expect("Failed to close database connection");

    init_database_dummy_data().await;
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

