use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use log::{debug, warn};

// Define the type for our registry
type ActionMap = Mutex<HashMap<String, Box<dyn Fn() + Send + Sync>>>;

// Use OnceLock to handle the static initialization safely
static ACTIONS: OnceLock<ActionMap> = OnceLock::new();

// Helper to get the registry, initializing it if necessary
fn get_actions() -> &'static ActionMap {
    ACTIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn register_action(id: String, f: impl Fn() + Send + Sync + 'static) {
    debug!("Register action: {}", id);
    get_actions().lock().unwrap().insert(id, Box::new(f));
}

pub fn clear_actions() {
    debug!("Clear all actions.");
    get_actions().lock().unwrap().clear();
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
