#![allow(non_snake_case)]
use tokio::sync::Mutex;
use tauri::{ Builder, State, Window };
use tokio::time::{ Duration, interval };
use tauri::Emitter;
use tauri::Manager;
use rand::{ Rng, SeedableRng, rngs::StdRng };
use std::iter::Enumerate;
use std::sync::Arc;
use chrono::Utc;

mod commands {
    pub mod stages;
}
struct AppData {
  welcome_message: String,
  count: i32,
  mostRecentStage: StageData, 
  telemetry: TelemetryData,
  mission: Mission,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct StageData {
  stageId: i32,
  stageName: String,
  vehicleName: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct TelemetryData {
  localIP: String,
  pitch: f32,
  yaw: f32,
  roll: f32,
  speed: f32,
  altitude: f32,
  batteryLife: f32,
  currentPosition: Coordinate,
  lastUpdated: chrono::DateTime<chrono::Utc>,
  fireFound: bool,
  fireCoordinate: Coordinate,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Mission {
  missionName: String,
  status: MissionStatus,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
enum MissionStatus {
  Active,
  Inactive,
  Complete,
  Failed,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Coordinate {
  latitude: f32,
  longitude: f32,
}

#[tauri::command]
async fn get_telemetry(
  app_data: State<'_, Arc<Mutex<AppData>>>,
  window: Window
) -> Result<(), String> {
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

// #[tauri::command]
// async fn set_most_recent_stage(
//   app_data: State<'_, Arc<Mutex<AppData>>>,
//   window: Window
// ) -> Result<(), String> {
//   let app_data = app_data.inner().clone();

//   tokio::spawn(async move {
//     let mut data = app_data.lock().await;

//     // get the most recent stage values
//     data.mostRecentStage.stageId += 1; // change this to just update with whatever new stage is made (will also have to remove out of loop)
//     data.mostRecentStage.stageName = format!("Stage {}", &data.mostRecentStage.stageId);
//     data.mostRecentStage.vehicleName = "FRA".to_string();

//     // Emit updated telemetry to frontend
//     if let Err(e) = window.emit("most_recent_stage_update", &data.mostRecentStage) {
//       eprintln!("Failed to emit most_recent_stage_update: {}", e);
//     }
//   });

//   Ok(())
// }

#[tauri::command]
async fn create_mission(
  app_data: State<'_, Arc<Mutex<AppData>>>,
  missionData: Mission,
  window: Window
) -> Result<(), String> {
  let app_data = app_data.inner().clone();
  
  tokio::spawn(async move {
    let mut data = app_data.lock().await;

    // post mission info
    data.mission.missionName = missionData.missionName;
    data.mission.status = missionData.status;
    
    // add in a query if postgres-ing

    // Emit updated telemetry to frontend
    if let Err(e) = window.emit("mission_post", "Posted MissionInfo") {
      eprintln!("Failed to emit mission_post: {}", e);
    }
  });

  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  Builder::default()
    .setup(|app| {
      app.manage(
        Arc::new(
          Mutex::new(AppData {
            welcome_message: "Welcome!".to_string(),
            count: 0,
            mostRecentStage: StageData {
              stageId: 0,
              stageName: "".to_string(),
              vehicleName: "".to_string(),
            },
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
            mission: Mission {
              missionName: "Mission 1".to_string(),
              status: MissionStatus::Active,
            },
          })
        )
      );
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_telemetry, commands::stages::set_most_recent_stage, commands::stages::transition_next_stage])
    .run(tauri::generate_context!())
    .expect("error while running Tauri application");
}
