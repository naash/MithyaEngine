// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{
    EntityManager, asset::AssetManager, core::Resources
};

pub struct World {
    pub resources: Resources,
    pub entity_manager: EntityManager,
    pub asset_manager: AssetManager,
}