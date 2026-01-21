use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

type StateMap = HashMap<TypeId, Arc<dyn Any + Send + Sync>>;

#[derive(Default, Clone)]
pub struct AppState {
    inner: StateMap,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn with<T: Send + Sync + 'static>(mut self, value: T) -> Self {
        self.inner.insert(TypeId::of::<T>(), Arc::new(value));
        self
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.inner
            .get(&TypeId::of::<T>())
            .and_then(|arc| arc.downcast_ref::<T>())
    }
}
