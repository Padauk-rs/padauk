use crate::ui::render_callback::request_redraw;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct State<T: Clone + Send + Sync + 'static> {
    inner: Arc<Mutex<T>>,
}

impl<T: Clone + Send + Sync + 'static> State<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value)),
        }
    }

    pub fn get(&self) -> T {
        self.inner.lock().unwrap().clone()
    }

    pub fn set(&self, value: T) {
        *self.inner.lock().unwrap() = value;
        request_redraw();
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        {
            let mut guard = self.inner.lock().unwrap();
            f(&mut *guard);
        }
        request_redraw();
    }
}

pub fn state<T: Clone + Send + Sync + 'static>(value: T) -> State<T> {
    State::new(value)
}
