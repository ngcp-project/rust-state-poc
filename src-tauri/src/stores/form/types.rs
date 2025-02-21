#[taurpc::ipc_type]
pub struct FormStateStruct {
    pub current_step: i32,
    pub total_steps: i32,
    pub form_data: FormDataStruct,
    pub is_submitted: bool,
}
// We can use nested structs in tauri state
// But must also decorate nested structs to be exported to bindings
#[taurpc::ipc_type]
#[derive(Debug)]
pub struct FormDataStruct {
    pub name: String,
    pub email: String,
    pub phone: String,
}