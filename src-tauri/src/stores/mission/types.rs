#[taurpc::ipc_type]
<<<<<<< HEAD
pub struct MissionStruct {
    pub mission_name: String,
    pub keep_in_zone: String,
    pub keep_out_zone: String,
    pub status: MissionStatus
}

#[taurpc::ipc_type]
pub struct MissionInfoStruct {
    pub current_mission: MissionStruct,
    pub mission_form_state: MissionStruct
}

#[derive(serde::Serialize, serde::Deserialize, specta::Type)]
#[derive(Clone)]
pub enum MissionStatus {
    Active,
    Inactive,
    Complete,
    Failed,
}

#[taurpc::ipc_type]
pub struct VehicleStruct {
    pub vehicle_name: String,
    pub current_stage_id: String,
    pub stages_list: Vec<MissionStageStruct>,
}
// stages_list[current_stage_id]


// Stage ID, vehicleName, searchArea, stageName, targetCoordinate
// 5, ERU, 1, Search and Rescue, 1.234, 5.678
// 7, MRA, 2, Search, 1.234, 5.678
// 10, MEU, 6, Rescue, 1.234, 5.678



#[taurpc::ipc_type]
pub struct MissionStageStruct {
    pub stage_name: String,
    pub vehicle_name: String,
    pub search_area: String,
    pub target_coordinate: String,
}

#[taurpc::ipc_type]
pub struct TestMissionStruct {
    pub stages: Vec<String>,
    pub current_stage_id: i32,
}

=======
// #[derive(Clone, serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
pub struct MissionStateStruct {
  pub current_step: i32,
  pub total_steps: i32,
  pub mission_data: MissionDataStruct,
  pub is_submitted: bool,
}

// We can use nested structs in tauri state
// But must also decorate nested structs to be exported to bindings

#[taurpc::ipc_type]
// #[derive(Clone, serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
pub struct MissionDataStruct {
  pub mission_name: String,
  pub keep_in_zone: String,
  pub keep_out_zone: String,
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
>>>>>>> ddf3fa73122a6792e740075c091d698cfe96a6c2
