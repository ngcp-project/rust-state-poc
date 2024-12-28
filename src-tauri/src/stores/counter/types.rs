#[derive(Debug)]
#[taurpc::ipc_type]
pub struct CounterStore {
    pub count: i32,
}