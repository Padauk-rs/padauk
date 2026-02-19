use log::{debug, warn};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// Define the type for our registry
type ActionMap = Mutex<HashMap<String, Box<dyn Fn() + Send + Sync>>>;
type ActionMapString = Mutex<HashMap<String, Box<dyn Fn(String) + Send + Sync>>>;

// Use OnceLock to handle the static initialization safely
static ACTIONS: OnceLock<ActionMap> = OnceLock::new();
static ACTIONS_STRING: OnceLock<ActionMapString> = OnceLock::new();

// Helper to get the registry, initializing it if necessary
fn get_actions() -> &'static ActionMap {
    ACTIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn get_actions_string() -> &'static ActionMapString {
    ACTIONS_STRING.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn register_action(id: String, f: impl Fn() + Send + Sync + 'static) {
    debug!("Register action: {}", id);
    get_actions().lock().unwrap().insert(id, Box::new(f));
}

pub fn register_action_with_string(id: String, f: impl Fn(String) + Send + Sync + 'static) {
    debug!("Register string action: {}", id);
    get_actions_string().lock().unwrap().insert(id, Box::new(f));
}

pub fn clear_actions() {
    debug!("Clear all actions.");
    get_actions().lock().unwrap().clear();
    get_actions_string().lock().unwrap().clear();
}

#[uniffi::export]
pub fn padauk_dispatch_action(id: String) {
    debug!("Dispatch action: {}", id);
    if let Some(f) = get_actions().lock().unwrap().get(&id) {
        f();
    } else {
        warn!("No action registered for id: {}", id);
    }
}

#[uniffi::export]
pub fn padauk_dispatch_action_with_string(id: String, payload: String) {
    debug!("Dispatch string action: {}", id);
    if let Some(f) = get_actions_string().lock().unwrap().get(&id) {
        f(payload);
    } else {
        warn!("No string action registered for id: {}", id);
    }
}
