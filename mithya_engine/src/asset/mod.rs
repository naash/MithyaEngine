// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod error;
pub mod managers;
pub mod asset_data;

pub use error::{AssetError, MaterialError};
pub use managers::AssetManager;
pub use asset_data::{MaterialData, UniformValue, TextureBinding};

