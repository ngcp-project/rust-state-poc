use std::sync::Arc; 
use tokio::sync::Mutex;
use tauri::{AppHandle, Wry};
use taurpc;

use super::types::TestMissionStruct;

// Define the FormApiImpl struct that contains a mutable FormStateStruct
#[derive(Clone)]
pub struct TestMissionAPI {
    state: Arc<Mutex<TestMissionStruct>>,
}


// Default implementation for FormApiImpl that sets the initial state
// Initializes form state when calling FormApiImpl::default()
impl Default for TestMissionAPI {
    fn default() -> Self {
        // Create a new initial state according to FormStateStruct
        let initial_state = TestMissionStruct {
            stages: vec!["Initialize".to_string(), "Search".to_string(), "Rescue".to_string()],
            current_stage_id: 0,
        };

        // Create a new instance of FormApiImpl with the initial state
        Self::new(initial_state)
    }
}

impl TestMissionAPI {
    // Constructor for FormApiImpl
    pub fn new(initial_state: TestMissionStruct) -> Self {
        // Must wrap the state in an Arc<Mutex<>>
        Self { 
            state: Arc::new(Mutex::new(initial_state)),
        }
    }
    // Helper method to emit state changes
    fn emit_state_update(&self, app_handle: &AppHandle<Wry>, state: &TestMissionStruct) -> Result<(), String> {
        MissionEventTrigger::new(app_handle.clone())
            .on_updated(state.clone())
            .map_err(|e| e.to_string())
    }

    // Helper method to modify state and emit update
    async fn update_state<F>(&self, app_handle: AppHandle<Wry>, updater: F) -> Result<(), String>
    where
        F: FnOnce(&mut TestMissionStruct)
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


pub trait MissionAPI {
    async fn transition_next_stage(app_handle: AppHandle<Wry>) -> Result<(), String>;
    async fn get_default_data() -> TestMissionStruct;
    async fn get_data() -> TestMissionStruct;

    #[taurpc(event)]
    async fn on_updated(new_data: TestMissionStruct);
}

// Implement the FormApi trait methods
#[taurpc::resolvers]
impl MissionAPI for TestMissionAPI {
    async fn get_default_data(self) -> TestMissionStruct {
        Self::default().state.lock().await.clone()
    }

    async fn transition_next_stage(self, app_handle: AppHandle<Wry>) -> Result<(), String> {
        self.update_state(app_handle, |state| {
            state.current_stage_id = (state.current_stage_id + 1) % state.stages.len() as i32;
            println!("Current Stage: {}", state.stages[state.current_stage_id as usize]);
        }).await
    }

    async fn get_data(self) -> TestMissionStruct {
        self.state.lock().await.clone()
    }
}