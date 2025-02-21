use std::sync::Arc; 
use tokio::sync::Mutex;
use tauri::{AppHandle, Wry};
use taurpc;

use super::types::{FormStateStruct, FormDataStruct};

// Define the FormApiImpl struct that contains a mutable FormStateStruct
#[derive(Clone)]
pub struct FormApiImpl {
    state: Arc<Mutex<FormStateStruct>>,
}


// Default implementation for FormApiImpl that sets the initial state
// Initializes form state when calling FormApiImpl::default()
impl Default for FormApiImpl {
    fn default() -> Self {
        // Create a new initial state according to FormStateStruct
        let initial_state = FormStateStruct {
            current_step: 1,
            total_steps: 2,
            form_data: FormDataStruct {
                name: "".to_string(),
                email: "".to_string(),
                phone: "".to_string(),
            },
            is_submitted: false,
        };

        // Create a new instance of FormApiImpl with the initial state
        Self::new(initial_state)
    }
}

impl FormApiImpl {
    // Constructor for FormApiImpl
    pub fn new(initial_state: FormStateStruct) -> Self {
        // Must wrap the state in an Arc<Mutex<>>
        Self { 
            state: Arc::new(Mutex::new(initial_state)),
        }
    }
    // Helper method to emit state changes
    fn emit_state_update(&self, app_handle: &AppHandle<Wry>, state: &FormStateStruct) -> Result<(), String> {
        FormEventTrigger::new(app_handle.clone())
            .on_updated(state.clone())
            .map_err(|e| e.to_string())
    }

    // Helper method to modify state and emit update
    async fn update_state<F>(&self, app_handle: AppHandle<Wry>, updater: F) -> Result<(), String>
    where
        F: FnOnce(&mut FormStateStruct)
    {
        let mut state = self.state.lock().await;

        // Updater is parameter that takes a generic function that can modify the state
        // state must be a mutable reference to the state
        updater(&mut state);
        // Emit the state update for frotnend to listen to
        self.emit_state_update(&app_handle, &state)
    }
}

#[taurpc::procedures(
    event_trigger = FormEventTrigger, // Define the event trigger for the form api (used in emit_state_update)
    export_to = "../src/lib/bindings.ts",  // Export the API to the bindings file
    path = "form" // Namespace for the form api
)]

// Define the FormApi trait with the required methods
pub trait FormApi {
    async fn next_step(app_handle: AppHandle<Wry>) -> Result<(), String>;
    async fn previous_step(app_handle: AppHandle<Wry>) -> Result<(), String>;
    async fn reset(app_handle: AppHandle<Wry>) -> Result<(), String>;
    async fn update_form(app_handle: AppHandle<Wry>, form_data: FormDataStruct) -> Result<(), String>;
    async fn submit_form(app_handle: AppHandle<Wry>) -> Result<(), String>;

    async fn get_default_data() -> FormStateStruct;
    async fn get_data() -> FormStateStruct;
    
    #[taurpc(event)]
    async fn on_updated(new_data: FormStateStruct);
}

// Implement the FormApi trait methods
#[taurpc::resolvers]
impl FormApi for FormApiImpl {
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
        // create a new FormStore with default values
        let default_state = Self::default().state.lock().await.clone();
        self.update_state(app_handle, |state| {
            // dereference the pointer(*) to directly modify state variable
            // in order to overwrite the existing state
            *state = default_state;
        }).await
    }

    async fn update_form(self, app_handle: AppHandle<Wry>, form_data: FormDataStruct) -> Result<(), String> {
        // |state| {...} is a closure (anonymous function) that takes a mutable reference to the state
        // wrap the parameters of closure in ||
        self.update_state(app_handle, |state| {
            // update only the nested form_data property
            state.form_data = form_data;
        }).await
    }

    async fn submit_form(self, app_handle: AppHandle<Wry>) -> Result<(), String> {
        self.update_state(app_handle, |state| {
            state.is_submitted = true;
            println!("Form submitted: {:?}", state.form_data);
        }).await
    }

    // Return the default state of the form
    // used by frontend to first initialize the form
    async fn get_default_data(self) -> FormStateStruct {
        Self::default().state.lock().await.clone()
    }
    
    // Get the current state of the form
    async fn get_data(self) -> FormStateStruct {
        self.state.lock().await.clone()
    }
}