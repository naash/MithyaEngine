// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec2;
use crate::physics::{
    physics_config::SEPARATION_BUFFER,
    components::ColliderShape
};

/// Information about a collision between two shapes
#[derive(Debug, Clone, Copy)]
pub struct CollisionInfo {
    /// Normal vector pointing from A to B
    pub normal: Vec2,
    /// Distance to separate the shapes
    pub separation: f32,
}

/// Check collision between two shapes and return collision info
pub fn detect_collision(pos_a: Vec2, shape_a: &ColliderShape, pos_b: Vec2, shape_b: &ColliderShape) -> Option<CollisionInfo> {
    use ColliderShape::*;

    match (shape_a, shape_b) {
        (Circle { radius: r1 }, Circle { radius: r2 }) => circle_vs_circle(pos_a, *r1, pos_b, *r2),
        (Box { width: w1, height: h1 }, Box { width: w2, height: h2 }) => box_vs_box(pos_a, *w1, *h1, pos_b, *w2, *h2),
        (Circle { radius: r }, Box { width, height }) => circle_vs_box(pos_a, *r, pos_b, *width, *height),
        (Box { width, height }, Circle { radius: r }) => {
            // Flip the result (normal points from A to B)
            circle_vs_box(pos_b, *r, pos_a, *width, *height)
                .map(|info| CollisionInfo { normal: -info.normal, separation: info.separation })
        }
    }
}

/// Circle-circle collision detection
pub fn circle_vs_circle(pos_a: Vec2, r1: f32, pos_b: Vec2, r2: f32) -> Option<CollisionInfo> {
    let diff = pos_a - pos_b;
    let distance = diff.length();
    let min_distance = r1 + r2;

    if distance >= min_distance {
        return None;
    }

    let normal = if distance > 0.0 { diff / distance } else { Vec2::new(1.0, 0.0) };
    let separation = (min_distance - distance + SEPARATION_BUFFER).max(0.0);
    Some(CollisionInfo { normal, separation })
}

/// Box-box collision detection (AABB)
pub fn box_vs_box(pos_a: Vec2, w1: f32, h1: f32, pos_b: Vec2, w2: f32, h2: f32) -> Option<CollisionInfo> {
    let diff = pos_a - pos_b;
    let overlap_x = (w1 + w2) / 2.0 - diff.x.abs();
    let overlap_y = (h1 + h2) / 2.0 - diff.y.abs();

    if overlap_x <= 0.0 || overlap_y <= 0.0 {
        return None;
    }

    let (normal, separation) = if overlap_x < overlap_y {
        (Vec2::new(diff.x.signum(), 0.0), overlap_x + SEPARATION_BUFFER)
    } else {
        (Vec2::new(0.0, diff.y.signum()), overlap_y + SEPARATION_BUFFER)
    };

    Some(CollisionInfo { normal, separation })
}

/// Circle-box collision detection
pub fn circle_vs_box(circle_pos: Vec2, radius: f32, box_pos: Vec2, box_width: f32, box_height: f32) -> Option<CollisionInfo> {
    let half_w = box_width / 2.0;
    let half_h = box_height / 2.0;

    let closest = Vec2::new(
        circle_pos.x.clamp(box_pos.x - half_w, box_pos.x + half_w),
        circle_pos.y.clamp(box_pos.y - half_h, box_pos.y + half_h),
    );

    let is_inside = closest == circle_pos;

    if is_inside {
        let dist_to_left = circle_pos.x - (box_pos.x - half_w);
        let dist_to_right = (box_pos.x + half_w) - circle_pos.x;
        let dist_to_bottom = circle_pos.y - (box_pos.y - half_h);
        let dist_to_top = (box_pos.y + half_h) - circle_pos.y;

        let min_dist = dist_to_left.min(dist_to_right).min(dist_to_bottom).min(dist_to_top);

        let normal = if min_dist == dist_to_left {
            Vec2::new(-1.0, 0.0)
        } else if min_dist == dist_to_right {
            Vec2::new(1.0, 0.0)
        } else if min_dist == dist_to_bottom {
            Vec2::new(0.0, -1.0)
        } else {
            Vec2::new(0.0, 1.0)
        };

        let separation = radius + min_dist;
        Some(CollisionInfo { normal, separation })
    } else {
        let to_circle = circle_pos - closest;
        let distance = to_circle.length();

        if distance >= radius {
            return None;
        }

        let normal = if distance > 0.0001 {
            to_circle / distance
        } else {
            let from_center = circle_pos - box_pos;
            if from_center.length() > 0.0001 {
                from_center.normalize()
            } else {
                Vec2::new(1.0, 0.0)
            }
        };

        let separation = (radius - distance).max(0.0);
        Some(CollisionInfo { normal, separation })
    }
}
