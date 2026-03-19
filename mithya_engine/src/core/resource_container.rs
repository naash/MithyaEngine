// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert<T: Any + Send + Sync + 'static>(&mut self, resource: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get<T: Any + Send + Sync + 'static>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>())
            .and_then(|r| r.downcast_ref())
    }

    pub fn get_mut<T: Any + Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.data.get_mut(&TypeId::of::<T>())
            .and_then(|r| r.downcast_mut())
    }

    pub fn remove<T: Any + Send + Sync + 'static>(&mut self) -> Option<T> {
        self.data.remove(&TypeId::of::<T>())
            .and_then(|r| r.downcast().ok())
            .map(|r| *r)
    }

    pub fn contains<T: Any + Send + Sync + 'static>(&self) -> bool {
        self.data.contains_key(&TypeId::of::<T>())
    }
}