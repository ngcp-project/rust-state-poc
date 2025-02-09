#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Builder};
use tokio::sync::Mutex;
use std::sync::Arc;
use serde_json::Value;

// The top-level application data contains vehicles, missions, and (optionally) a stages array.
struct AppData {
    vehicle: Vehicle,
}

// given: currentStageId = 2
// Example: A vehicle has multiple stages; the current_stage field holds the ID (e.g. 2.0),
// and the stages vector holds all Stage structs.
impl AppData {
    fn new() -> Self {
        Self {
            vehicle: 
                Vehicle {
                    vehicle_name: "Vehicle 1".to_string(),
                    mission_name: "Mission Alpha".to_string(),
                    current_stage: 2, // current stage id is 2
                    stages: vec![
                        Stage {
                            stage_id: 1,
                            stage_name: "Stage 1".to_string(),
                            vehicle_name: "Vehicle 1".to_string(),
                            zones: Zones {
                                keep_in_zone: vec![
                                    Coordinate { latitude: -1.0, longitude: 2.0 },
                                    Coordinate { latitude: 3.0, longitude: -2.0 },
                                ],
                                keep_out_zone: vec![
                                    Coordinate { latitude: -1.0, longitude: 2.0 },
                                    Coordinate { latitude: 3.0, longitude: -2.0 },
                                ],
                                search_area: vec![
                                    Coordinate { latitude: -1.0, longitude: 2.0 },
                                    Coordinate { latitude: 3.0, longitude: -2.0 },
                                ],
                                target_coordinate: Coordinate { latitude: 0.0, longitude: 0.0 },
                                stage_id: 1,
                            },
                        },
                        Stage {
                            stage_id: 2,
                            stage_name: "Stage 2".to_string(),
                            vehicle_name: "Vehicle 1".to_string(),
                            zones: Zones {
                                keep_in_zone: vec![
                                    Coordinate { latitude: -1.7, longitude: 2.2 },
                                    Coordinate { latitude: 3.2, longitude: -2.1 },
                                ],
                                keep_out_zone: vec![
                                    Coordinate { latitude: -1.7, longitude: 2.2 },
                                    Coordinate { latitude: 3.2, longitude: -2.1 },
                                ],
                                search_area: vec![
                                    Coordinate { latitude: -1.7, longitude: 2.2 },
                                    Coordinate { latitude: 3.2, longitude: -2.1 },
                                ],
                                target_coordinate: Coordinate { latitude: 0.0, longitude: 0.0 },
                                stage_id: 2,
                            },
                        },
                    ],
                },
            // missions: vec![
            //     Mission {
            //         mission_name: "Mission Alpha".to_string(),
            //         status: "Active".to_string(),
            //     },
            // ],
            // // You can also populate a top-level stages vector if needed.
            // stages: vec![],
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Coordinate {
    latitude: f32,
    longitude: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Mission {
    mission_name: String,
    status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Stage {
    stage_id: i32, // Primary key
    stage_name: String,
    vehicle_name: String, // Foreign key reference to Vehicle
    zones: Zones,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Vehicle {
    vehicle_name: String, 
    mission_name: String,
    current_stage: i32, // Only the current stage ID
    stages: Vec<Stage>, // All stages available for this vehicle
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Zones {
    keep_in_zone: Vec<Coordinate>,
    keep_out_zone: Vec<Coordinate>,
    search_area: Vec<Coordinate>,
    target_coordinate: Coordinate,
    stage_id: i32,
}

#[tauri::command]
async fn get_vehicle_data(
    app_data: tauri::State<'_, Arc<Mutex<AppData>>>,
    vehicle_name: String
) -> Result<Value, String> {
    println!("Received request for vehicle_name: {}", vehicle_name); // Debugging statement

    let data = app_data.lock().await;

    // Since there's only one vehicle, directly compare its name
    if data.vehicle.vehicle_name == vehicle_name {
        println!("Vehicle: {:?}", data.vehicle); // Debugging statement
        return Ok(serde_json::json!(data.vehicle));
    } else {
        println!("Vehicle '{}' not found", vehicle_name); // Debugging statement
        return Err(format!("Vehicle '{}' not found", vehicle_name));
    
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(AppData::new())))
        .invoke_handler(tauri::generate_handler![get_vehicle_data])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
