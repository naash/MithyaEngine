// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[derive(Debug, Clone)]
pub struct Controller {
    pub possessed_entity_id: u32,
}

impl Controller {
    pub fn new(possessed_entity_id: u32) -> Self {
        Self { possessed_entity_id }
    }
}