use crate::ui::{render_callback::request_redraw, widget::Widget};
use std::sync::{Arc, Mutex, OnceLock}; // Assuming State is available in lib.rs

// A Route is a named builder for a Widget (Page)
#[derive(Clone)]
pub struct Route {
    pub name: String,
    // We use Arc<dyn Fn> to make the builder cloneable and thread-safe
    pub builder: Arc<dyn Fn() -> Box<dyn Widget> + Send + Sync>,
}

impl Route {
    pub fn new<F, W>(name: impl Into<String>, builder: F) -> Self
    where
        F: Fn() -> W + Send + Sync + 'static,
        W: Widget + 'static,
    {
        Self {
            name: name.into(),
            builder: Arc::new(move || Box::new(builder())),
        }
    }
}

struct NavigatorState {
    stack: Vec<Route>,
}

// Global Singleton Navigator State
static NAVIGATOR_STATE: OnceLock<Mutex<NavigatorState>> = OnceLock::new();

pub struct Navigator;

impl Navigator {
    // Initialize the navigator with a root route.
    /// This should be called once at app startup.
    pub fn init(initial_route: Route) {
        let state = NavigatorState {
            stack: vec![initial_route],
        };
        // We ignore the error if it's already initialized
        let _ = NAVIGATOR_STATE.set(Mutex::new(state));
    }

    /// Push a new route onto the stack
    pub fn push(route: Route) {
        if let Some(mutex) = NAVIGATOR_STATE.get() {
            if let Ok(mut state) = mutex.lock() {
                state.stack.push(route);
                request_redraw();
            }
        }
    }

    /// Pop the top route from the stack
    pub fn pop() {
        if let Some(mutex) = NAVIGATOR_STATE.get() {
            if let Ok(mut state) = mutex.lock() {
                if state.stack.len() > 1 {
                    state.stack.pop();
                    request_redraw();
                }
            }
        }
    }

    /// Check if there is more than one route in the stack
    pub fn can_pop() -> bool {
        if let Some(mutex) = NAVIGATOR_STATE.get() {
            if let Ok(state) = mutex.lock() {
                return state.stack.len() > 1;
            }
        }
        false
    }

    /// Render the currently active route
    pub fn render_current() -> Option<Box<dyn Widget>> {
        if let Some(mutex) = NAVIGATOR_STATE.get() {
            if let Ok(state) = mutex.lock() {
                if let Some(route) = state.stack.last() {
                    return Some((route.builder)());
                }
            }
        }
        None
    }
}
