use std::sync::Arc; 
use tokio::sync::Mutex;
use tauri::{AppHandle, Wry};
use taurpc;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use sqlx::query;

use super::types::{ MissionInfoStruct, MissionStruct, MissionStatus };

// Define the FormApiImpl struct that contains a mutable FormStateStruct
#[derive(Clone)]
pub struct MissionApiImpl {
    state: Arc<Mutex<MissionInfoStruct>>,
    db: PgPool,
}

impl MissionApiImpl {
    // Constructor for FormApiImpl
    pub async fn new() -> Self {
        let initial_state = MissionInfoStruct {
            stages: vec!["Initialize".to_string(), "Search".to_string(), "Rescue".to_string()],
            current_stage_id: 0,
            current_mission: MissionStruct {
                mission_name: "".to_string(),
                keep_out_zone: "".to_string(),
                keep_in_zone: "".to_string(),
                status: MissionStatus::Inactive,
            },
            mission_form_state: MissionStruct {
                mission_name: "".to_string(),
                keep_out_zone: "".to_string(),
                keep_in_zone: "".to_string(),
                status: MissionStatus::Inactive,
            },
            is_submitted: false,
        };


        // Must wrap the state in an Arc<Mutex<>>
        Self { 
            state: Arc::new(Mutex::new(initial_state)),
            db: PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://ngcp:ngcp@localhost:5433/ngcpdb")
                .await
                .expect("Failed to connect to the database"),
        }
    }
    // Helper method to emit state changes
    fn emit_state_update(&self, app_handle: &AppHandle<Wry>, state: &MissionInfoStruct) -> Result<(), String> {
        MissionEventTrigger::new(app_handle.clone())
            .on_updated(state.clone())
            .map_err(|e| e.to_string())
    }

    // Helper method to modify state and emit update
    async fn update_state<F>(&self, app_handle: AppHandle<Wry>, updater: F) -> Result<(), String>
    where
        F: FnOnce(&mut MissionInfoStruct)
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
    event_trigger = MissionEventTrigger, // Define the event trigger for the form api (used in emit_state_update)
    export_to = "../src/lib/bindings.ts", 
    path = "mission" // Namespace for the form api
)]


pub trait MissionApi {
    async fn transition_next_stage(app_handle: AppHandle<Wry>) -> Result<(), String>;
    async fn get_default_data() -> MissionInfoStruct;
    async fn get_data() -> MissionInfoStruct;
    async fn reset(app_handle: AppHandle<Wry>) -> Result<(), String>;
    async fn submit_mission(app_handle: AppHandle<Wry>) -> Result<(), String>;
    async fn update_mission_data(app_handle: AppHandle<Wry>, mission_data: MissionStruct) -> Result<(), String>;

    #[taurpc(event)]
    async fn on_updated(new_data: MissionInfoStruct);
}

// Implement the FormApi trait methods
#[taurpc::resolvers]
impl MissionApi for MissionApiImpl {
    async fn get_default_data(self) -> MissionInfoStruct {
        Self::new().await.state.lock().await.clone()
    }

    async fn transition_next_stage(self, app_handle: AppHandle<Wry>) -> Result<(), String> {
        query("
            INSERT INTO test (name, age) VALUES ($1, $2)
        ").bind("John Doe")
        .bind(30)
        .execute(&self.db).await.expect("Failed to insert stage");

        self.update_state(app_handle, |state| {
            state.current_stage_id = (state.current_stage_id + 1) % state.stages.len() as i32;
            println!("Current Stage: {}", state.stages[state.current_stage_id as usize]);
        }).await
    }

    async fn get_data(self) -> MissionInfoStruct {
        self.state.lock().await.clone()
    }


    async fn reset(self, app_handle: AppHandle<Wry>) -> Result<(), String> {
      // create a new MissionStruct with default values
      let default_state = Self::new().await.state.lock().await.clone();
      self.update_state(app_handle, |state| {
        // dereference the pointer(*) to directly modify state variable
        // in order to overwrite the existing state
        *state = default_state;
      }).await
    }
  
    async fn update_mission_data(
      self,
      app_handle: AppHandle<Wry>,
      mission_data: MissionStruct
    ) -> Result<(), String> {
      // |state| {...} is a closure (anonymous function) that takes a mutable reference to the state
      // wrap the parameters of closure in ||
      self.update_state(app_handle, |state| {
        // update only the nested mission_data property
        state.mission_form_state = mission_data;
      }).await
    }
  
    async fn submit_mission(self, app_handle: AppHandle<Wry>) -> Result<(), String> {
      self.update_state(app_handle, |state| {
        state.is_submitted = true;
        println!("Mission submitted: {:?}", state.mission_form_state);
      }).await
    }
}
