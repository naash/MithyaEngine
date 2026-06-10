// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::{Mat4, Vec2, Vec3, Vec4};

/// Updated by RenderingSystem every frame with the active camera's matrices
/// and the surface size in physical pixels — the same space winit reports
/// cursor positions in, so screen_to_world is DPI-scale independent.
pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub view: Mat4,
    pub projection: Mat4,
}

impl Viewport {
    pub fn screen_to_world(&self, screen: Vec2) -> Vec3 {
        let ndc = Vec2::new(
            (screen.x / self.width as f32) * 2.0 - 1.0,
            1.0 - (screen.y / self.height as f32) * 2.0,
        );
        let world = (self.projection * self.view).inverse()
            * Vec4::new(ndc.x, ndc.y, 0.0, 1.0);
        Vec3::new(world.x, world.y, world.z) / world.w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ortho_viewport(width: u32, height: u32, half_height: f32, camera_pos: Vec3) -> Viewport {
        let aspect = width as f32 / height as f32;
        Viewport {
            width,
            height,
            view: Mat4::from_translation(camera_pos).inverse(),
            projection: Mat4::orthographic_rh(
                -half_height * aspect, half_height * aspect,
                -half_height, half_height,
                -1.0, 1.0,
            ),
        }
    }

    #[test]
    fn screen_center_maps_to_camera_position() {
        let viewport = ortho_viewport(1920, 1080, 5.0, Vec3::new(3.0, -2.0, 0.0));
        let world = viewport.screen_to_world(Vec2::new(960.0, 540.0));
        assert!((world.x - 3.0).abs() < 1e-4);
        assert!((world.y + 2.0).abs() < 1e-4);
    }

    #[test]
    fn screen_corners_map_to_camera_extents() {
        let viewport = ortho_viewport(1920, 1080, 5.0, Vec3::ZERO);
        let aspect = 1920.0 / 1080.0;

        let top_left = viewport.screen_to_world(Vec2::ZERO);
        assert!((top_left.x + 5.0 * aspect).abs() < 1e-3);
        assert!((top_left.y - 5.0).abs() < 1e-3);

        let bottom_right = viewport.screen_to_world(Vec2::new(1920.0, 1080.0));
        assert!((bottom_right.x - 5.0 * aspect).abs() < 1e-3);
        assert!((bottom_right.y + 5.0).abs() < 1e-3);
    }

    #[test]
    fn mapping_is_independent_of_dpi_scale() {
        // Same window at 100% and 200% display scale: physical size doubles,
        // and so do physical cursor coordinates for the same point on screen.
        let logical = ortho_viewport(960, 540, 5.0, Vec3::ZERO);
        let scaled = ortho_viewport(1920, 1080, 5.0, Vec3::ZERO);

        let a = logical.screen_to_world(Vec2::new(240.0, 135.0));
        let b = scaled.screen_to_world(Vec2::new(480.0, 270.0));
        assert!((a - b).length() < 1e-4);
    }
}
