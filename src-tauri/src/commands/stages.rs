use tokio::sync::Mutex;
use tauri::{ window, Builder, State, Window };
use std::sync::Arc;
use tauri::Emitter;

pub struct AppData {
  mostRecentStage: StageData,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct StageData {
  stageId: i32,
  stageName: String,
  vehicleName: String,
}

#[tauri::command]
pub async fn transition_next_stage() {
    println!("Transition Next Stage!");
}

#[tauri::command]
pub async fn set_most_recent_stage(app_data: State<'_, Arc<Mutex<AppData>>>, window: Window) -> Result<(), String> {
  let app_data = app_data.inner().clone();

  tokio::spawn(async move {
    let mut data = app_data.lock().await;

    // get the most recent stage values
    data.mostRecentStage.stageId += 1; // change this to just update with whatever new stage is made (will also have to remove out of loop)
    data.mostRecentStage.stageName = format!("Stage {}", &data.mostRecentStage.stageId);
    data.mostRecentStage.vehicleName = "FRA".to_string();

    // Emit updated telemetry to frontend
    if let Err(e) = window.emit("most_recent_stage_update", &data.mostRecentStage) {
      eprintln!("Failed to emit most_recent_stage_update: {}", e);
    }
  });

  Ok(())
}