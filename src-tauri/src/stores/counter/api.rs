use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{AppHandle, Wry};
use taurpc;

use super::types::CounterStore;


// Define CounterApiImpl as a struct that contains a mutable CounterStore
#[derive(Clone)]
pub struct CounterApiImpl {
    // Must use a state key for thread safety
    // and wrap the store state type in an Arc<Mutex<>>
    state: Arc<Mutex<CounterStore>>,
}

// Default implementation for CounterApiImpl that sets the initial count to 0
impl Default for CounterApiImpl {
    fn default() -> Self {
        let initial_state = CounterStore {
            count: 0,
        };

        Self::new(initial_state)
    }
}


// Constructor for CounterApiImpl
impl CounterApiImpl {
    pub fn new(initial_state: CounterStore) -> Self {
        Self { 
            state: Arc::new(Mutex::new(initial_state)),
        }
    }
    // Helper method to emit state changes
    fn emit_state_update(&self, app_handle: &AppHandle<Wry>, state: &CounterStore) -> Result<(), String> {
        CounterEventTrigger::new(app_handle.clone())
            .on_updated(state.clone())
            .map_err(|e| e.to_string())
    }

    // Helper method to modify state and emit update
    async fn update_state<F>(&self, app_handle: AppHandle<Wry>, updater: F) -> Result<(), String>
    where
        F: FnOnce(&mut CounterStore)
    {
        // Lock the variable to modify
        let mut state = self.state.lock().await;

        // Updater is generic function that can modify the state
        // state must be a mutable reference to the state
        updater(&mut state);
        self.emit_state_update(&app_handle, &state)
    }
}

// Define the CounterApi trait with the required methods
#[taurpc::procedures(
    event_trigger = CounterEventTrigger, // Define the event trigger for the counter api
    export_to = "../src/lib/bindings.ts",  // Export the API to the bindings file
    path = "counter" // Namespace for the counter api
)]
pub trait CounterApi {
    async fn increase(app_handle: AppHandle<Wry>) -> Result<(), String>;

    async fn get_data() -> CounterStore;
    #[taurpc(event)] // Define the on_updated method as an event for frontend to listen to
    async fn on_updated(new_data: CounterStore);
}

// Define the resolvers for the CounterApi trait
#[taurpc::resolvers]
impl CounterApi for CounterApiImpl {
    // implement increase method
    async fn increase(self, app_handle: AppHandle<Wry>) -> Result<(), String> {
        // wrap the 
        self.update_state(app_handle, |state| {
            state.count += 1;
            println!("Count increased: {}", state.count);
        }).await
    }

    // Get the current state of the counter
    async fn get_data(self) -> CounterStore {
        self.state.lock().await.clone()
    }
}
