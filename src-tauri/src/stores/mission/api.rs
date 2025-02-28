use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{ AppHandle, Wry };
use taurpc;

use super::types::{ MissionStateStruct, MissionDataStruct, MissionStatus };

// Define the MissionApiImpl struct that contains a mutable MissionStateStruct
#[derive(Clone)]
pub struct MissionApiImpl {
  state: Arc<Mutex<MissionStateStruct>>,
}

// Default implementation for MissionApiImpl that sets the initial state
// Initializes mission state when calling MissionApiImpl::default()
impl Default for MissionApiImpl {
  fn default() -> Self {
    // Create a new initial state according to MissionStateStruct
    let initial_state = MissionStateStruct {
      current_step: 1,
      total_steps: 2,
      keep_in_zone_coord: "".to_string(),
      keep_out_zone_coord: "".to_string(),
      mission_data: MissionDataStruct {
        mission_name: "".to_string(),
        keep_out_zone: Vec::new(),
        keep_in_zone: Vec::new(),
        status: MissionStatus::Inactive,
      },
      is_submitted: false,
    };

    // Create a new instance of MissionApiImpl with the initial state
    Self::new(initial_state)
  }
}

impl MissionApiImpl {
  // Constructor for MissionApiImpl
  pub fn new(initial_state: MissionStateStruct) -> Self {
    // Must wrap the state in an Arc<Mutex<>>
    Self {
      state: Arc::new(Mutex::new(initial_state)),
    }
  }

  // Helper method to emit state changes
  fn emit_state_update(
    &self,
    app_handle: &AppHandle<Wry>,
    state: &MissionStateStruct
  ) -> Result<(), String> {
    MissionEventTrigger::new(app_handle.clone())
      .on_updated(state.clone())
      .map_err(|e| e.to_string())
  }

  // Helper method to modify state and emit update
  async fn update_state<F>(&self, app_handle: AppHandle<Wry>, updater: F) -> Result<(), String>
    where F: FnOnce(&mut MissionStateStruct)
  {
    let mut state = self.state.lock().await;

    // Updater is parameter that takes a generic function that can modify the state
    // state must be a mutable reference to the state
    updater(&mut state);
    // Emit the state update for frontend to listen to
    self.emit_state_update(&app_handle, &state)
  }
}

#[taurpc::procedures(
  event_trigger = MissionEventTrigger, // Define the event trigger for the mission api (used in emit_state_update)
  export_to = "../src/lib/bindings.ts", // Export the API to the bindings file
  path = "mission" // Namespace for the mission api
)]

// Define the MissionApi trait with the required methods
pub trait MissionApi {
  async fn next_step(app_handle: AppHandle<Wry>) -> Result<(), String>;
  async fn previous_step(app_handle: AppHandle<Wry>) -> Result<(), String>;
  async fn reset(app_handle: AppHandle<Wry>) -> Result<(), String>;
  async fn update_mission_data(
    app_handle: AppHandle<Wry>,
    mission_data: MissionDataStruct
  ) -> Result<(), String>;
  async fn append_keep_in_out_zone_coords(
    app_handle: AppHandle<Wry>,
    keep_in_zone: String,
    keep_out_zone: String
  ) -> Result<(), String>;
  async fn submit_mission(app_handle: AppHandle<Wry>) -> Result<(), String>;

  async fn get_default_data() -> MissionStateStruct;
  async fn get_data() -> MissionStateStruct;

  #[taurpc(event)]
  async fn on_updated(new_data: MissionStateStruct);
}

// Implement the MissionApi trait methods
#[taurpc::resolvers]
impl MissionApi for MissionApiImpl {
  async fn next_step(self, app_handle: AppHandle<Wry>) -> Result<(), String> {
    self.update_state(app_handle, |state| {
      if state.current_step < state.total_steps {
        state.current_step += 1;
      } else {
        println!("Reached last step");
      }

      println!("On step {}", state.current_step);
    }).await
  }

  async fn previous_step(self, app_handle: AppHandle<Wry>) -> Result<(), String> {
    self.update_state(app_handle, |state| {
      if state.current_step > 1 {
        state.current_step -= 1;
      } else {
        println!("Reached first step");
      }

      println!("On step {}", state.current_step);
    }).await
  }

  async fn reset(self, app_handle: AppHandle<Wry>) -> Result<(), String> {
    // create a new MissionStateStruct with default values
    let default_state = Self::default().state.lock().await.clone();
    self.update_state(app_handle, |state| {
      // dereference the pointer(*) to directly modify state variable
      // in order to overwrite the existing state
      *state = default_state;
    }).await
  }

  async fn update_mission_data(
    self,
    app_handle: AppHandle<Wry>,
    mission_data: MissionDataStruct
  ) -> Result<(), String> {
    // |state| {...} is a closure (anonymous function) that takes a mutable reference to the state
    // wrap the parameters of closure in ||
    self.update_state(app_handle, |state| {
      // update only the nested mission_data property
      state.mission_data = mission_data;
    }).await
  }

  async fn append_keep_in_out_zone_coords(
    self,
    app_handle: AppHandle<Wry>,
    keep_in_zone_coords: String,
    keep_out_zone_coords: String
  ) -> Result<(), String> {
    self.update_state(app_handle, |state| {
      state.mission_data.keep_in_zone.push(keep_in_zone_coords);
      state.mission_data.keep_out_zone.push(keep_out_zone_coords);
      println!("Keep in zone: {:?}", state.mission_data.keep_in_zone);
      println!("Keep out zone: {:?}", state.mission_data.keep_out_zone);
    }).await
  }

  async fn submit_mission(self, app_handle: AppHandle<Wry>) -> Result<(), String> {
    self.update_state(app_handle, |state| {
      state.is_submitted = true;
      println!("Mission submitted: {:?}", state.mission_data);
    }).await
  }

  // Return the default state of the mission
  // used by frontend to first initialize the mission
  async fn get_default_data(self) -> MissionStateStruct {
    Self::default().state.lock().await.clone()
  }

  // Get the current state of the mission
  async fn get_data(self) -> MissionStateStruct {
    self.state.lock().await.clone()
  }
}
