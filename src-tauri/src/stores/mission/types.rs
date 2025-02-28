#[taurpc::ipc_type]
// #[derive(Clone, serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
pub struct MissionStateStruct {
  pub current_step: i32,
  pub total_steps: i32,
  pub mission_data: MissionDataStruct,
  pub keep_in_zone_coord: String,
  pub keep_out_zone_coord: String,
  pub is_submitted: bool,
}

// We can use nested structs in tauri state
// But must also decorate nested structs to be exported to bindings


#[taurpc::ipc_type]
// #[derive(Clone, serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
pub struct MissionDataStruct {
  pub mission_name: String,
  pub keep_in_zone: Vec<String>,
  pub keep_out_zone: Vec<String>,
  pub status: MissionStatus,
}

// #[derive(Clone, serde::Serialize, serde::Deserialize)]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type)]
// #[derive(Debug)]
pub enum MissionStatus {
    Active,
    Inactive,
    Complete,
    Failed,
  }
