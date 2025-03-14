#[taurpc::ipc_type]
pub struct MissionInfoStruct {
    pub current_mission: MissionStruct,
    pub mission_form_state: MissionStruct,
    pub stages: Vec<String>,
    pub current_stage_id: i32,
    pub is_submitted: bool,
}

#[taurpc::ipc_type]
#[derive(Debug)]
pub struct MissionStruct {
    pub mission_name: String,
    pub keep_in_zone: String,
    pub keep_out_zone: String,
    pub status: MissionStatus
}

#[derive(serde::Serialize, serde::Deserialize, specta::Type, Clone, Debug)]
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
