#![allow(non_snake_case)]
use tokio::sync::Mutex;
use tauri::{Builder, State, Window};
use tokio::time::{Duration, interval};
use tauri::Emitter;
use tauri::Manager;
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::sync::Arc;
use chrono::Utc;

struct AppData {
    welcome_message: String,
    count: i32,
    telemetry: TelemetryData,
}


#[derive(Clone, serde::Serialize. serde::Deserialize)]
struct VehicleData{
    vehicleName: String,
    isManual: bool,
    target: Coordinate,
    searchArea: Coordinate
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Coordinate {
    latitude: f32,
    longitude: f32,
}

#[tauri::command]
async fn get_telemetry(app_data: State<'_, Arc<Mutex<AppData>>>, window: Window) -> Result<(), String> {
    let mut interval = interval(Duration::from_secs(1));
    let app_data = app_data.inner().clone();

    tokio::spawn(async move {
        // Create a Send-safe random number generator
        let mut rng = StdRng::from_entropy(); // StdRng is Send and can be used in async tasks
        
        loop {
            interval.tick().await;

            let mut data = app_data.lock().await;
            
            // Randomize telemetry values
            data.telemetry.speed += rng.gen_range(0.5..2.0); // Random speed increase (between 0.5 and 2.0)
            data.telemetry.altitude += rng.gen_range(-1.0..1.0); // Random altitude fluctuation (-1.0 to 1.0)
            data.telemetry.pitch += rng.gen_range(-0.5..0.5); // Random pitch fluctuation (-0.5 to 0.5)
            data.telemetry.yaw += rng.gen_range(-1.0..1.0); // Random yaw fluctuation (-1.0 to 1.0)
            data.telemetry.roll += rng.gen_range(-0.5..0.5); // Random roll fluctuation (-0.5 to 0.5)

            // Update the last updated timestamp
            data.telemetry.lastUpdated = Utc::now();

            // Emit updated telemetry to frontend
            if let Err(e) = window.emit("telemetry_update", &data.telemetry) {
                eprintln!("Failed to emit telemetry update: {}", e);
            }
        }
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.manage(Arc::new(Mutex::new(AppData {
                welcome_message: "Welcome!".to_string(),
                count: 0,
                telemetry: TelemetryData {
                    localIP: "192.168.1.1".to_string(),
                    pitch: 0.0,
                    yaw: 0.0,
                    roll: 0.0,
                    speed: 0.0,
                    altitude: 0.0,
                    batteryLife: 100.0,
                    currentPosition: Coordinate {
                        latitude: 37.7749,
                        longitude: -122.4194,
                    },
                    lastUpdated: Utc::now(),
                    fireFound: false,
                    fireCoordinate: Coordinate {
                        latitude: 0.0,
                        longitude: 0.0,
                    },
                },
            })));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_telemetry])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
