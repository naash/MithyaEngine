// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec2;
use crate::physics::{collision_config::SEPARATION_BUFFER, components::ColliderShape};

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

/// Check if two circles are colliding
pub fn circles_colliding(pos_a: Vec2, r1: f32, pos_b: Vec2, r2: f32) -> bool {
    let distance_sq = (pos_a - pos_b).length_squared();
    let min_distance = r1 + r2;
    distance_sq < min_distance * min_distance
}

/// Check if two boxes (AABB) are colliding
pub fn boxes_colliding(pos_a: Vec2, w1: f32, h1: f32, pos_b: Vec2, w2: f32, h2: f32) -> bool {
    let diff = pos_a - pos_b;
    let overlap_x = (w1 + w2) / 2.0 - diff.x.abs();
    let overlap_y = (h1 + h2) / 2.0 - diff.y.abs();
    overlap_x > 0.0 && overlap_y > 0.0
}

/// Check if circle and box are colliding
pub fn circle_box_colliding(circle_pos: Vec2, radius: f32, box_pos: Vec2, box_width: f32, box_height: f32) -> bool {
    let closest = closest_point_on_box(box_pos, circle_pos, box_width, box_height);
    let distance_sq = (circle_pos - closest).length_squared();
    distance_sq < radius * radius
}

/// Get closest point on a box to a given point
pub fn closest_point_on_box(box_pos: Vec2, point: Vec2, width: f32, height: f32) -> Vec2 {
    let half_w = width / 2.0;
    let half_h = height / 2.0;
    Vec2::new(
        point.x.clamp(box_pos.x - half_w, box_pos.x + half_w),
        point.y.clamp(box_pos.y - half_h, box_pos.y + half_h),
    )
}

/// Check if a point is inside a box
pub fn point_in_box(point: Vec2, box_pos: Vec2, width: f32, height: f32) -> bool {
    let half_w = width / 2.0;
    let half_h = height / 2.0;
    point.x >= box_pos.x - half_w && point.x <= box_pos.x + half_w && point.y >= box_pos.y - half_h && point.y <= box_pos.y + half_h
}

/// Check if a point is inside a circle
pub fn point_in_circle(point: Vec2, circle_pos: Vec2, radius: f32) -> bool {
    (point - circle_pos).length_squared() <= radius * radius
}

/// Calculate distance between two points
pub fn distance(a: Vec2, b: Vec2) -> f32 {
    (b - a).length()
}

/// Calculate squared distance between two points (faster, no sqrt)
pub fn distance_squared(a: Vec2, b: Vec2) -> f32 {
    (b - a).length_squared()
}

/// Get direction from one point to another (normalized)
pub fn direction(from: Vec2, to: Vec2) -> Vec2 {
    (to - from).normalize_or_zero()
}

// ============================================================================
// Ray Casting (Useful for AI line-of-sight)
// ============================================================================

/// Ray-box intersection test
pub fn ray_intersects_box(ray_origin: Vec2, ray_direction: Vec2, box_pos: Vec2, box_width: f32, box_height: f32) -> Option<f32> {
    let half_w = box_width / 2.0;
    let half_h = box_height / 2.0;

    let box_min = Vec2::new(box_pos.x - half_w, box_pos.y - half_h);
    let box_max = Vec2::new(box_pos.x + half_w, box_pos.y + half_h);

    let inv_dir = Vec2::new(1.0 / ray_direction.x, 1.0 / ray_direction.y);

    let t1 = (box_min.x - ray_origin.x) * inv_dir.x;
    let t2 = (box_max.x - ray_origin.x) * inv_dir.x;
    let t3 = (box_min.y - ray_origin.y) * inv_dir.y;
    let t4 = (box_max.y - ray_origin.y) * inv_dir.y;

    let tmin = t1.min(t2).max(t3.min(t4));
    let tmax = t1.max(t2).min(t3.max(t4));

    if tmax < 0.0 || tmin > tmax {
        None
    } else {
        Some(tmin.max(0.0))
    }
}

/// Ray-circle intersection test
pub fn ray_intersects_circle(ray_origin: Vec2, ray_direction: Vec2, circle_pos: Vec2, radius: f32) -> Option<f32> {
    let to_circle = circle_pos - ray_origin;
    let proj = to_circle.dot(ray_direction);

    if proj < 0.0 {
        return None;
    }

    let closest_point = ray_origin + ray_direction * proj;
    let distance_sq = (circle_pos - closest_point).length_squared();

    if distance_sq > radius * radius {
        None
    } else {
        let offset = (radius * radius - distance_sq).sqrt();
        Some((proj - offset).max(0.0))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_collision() {
        assert!(circles_colliding(Vec2::new(0.0, 0.0), 5.0, Vec2::new(8.0, 0.0), 5.0));
        assert!(!circles_colliding(Vec2::new(0.0, 0.0), 5.0, Vec2::new(20.0, 0.0), 5.0));
    }

    #[test]
    fn test_box_collision() {
        assert!(boxes_colliding(Vec2::new(0.0, 0.0), 10.0, 10.0, Vec2::new(5.0, 0.0), 10.0, 10.0));
        assert!(!boxes_colliding(Vec2::new(0.0, 0.0), 10.0, 10.0, Vec2::new(20.0, 0.0), 10.0, 10.0));
    }

    #[test]
    fn test_point_in_shapes() {
        assert!(point_in_circle(Vec2::new(3.0, 0.0), Vec2::new(0.0, 0.0), 5.0));
        assert!(!point_in_circle(Vec2::new(10.0, 0.0), Vec2::new(0.0, 0.0), 5.0));

        assert!(point_in_box(Vec2::new(3.0, 3.0), Vec2::new(0.0, 0.0), 10.0, 10.0));
        assert!(!point_in_box(Vec2::new(10.0, 10.0), Vec2::new(0.0, 0.0), 10.0, 10.0));
    }

    #[test]
    fn test_ray_intersections() {
        assert!(ray_intersects_box(Vec2::new(-10.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 0.0), 5.0, 5.0).is_some());
        assert!(ray_intersects_box(Vec2::new(-10.0, 0.0), Vec2::new(-1.0, 0.0), Vec2::new(0.0, 0.0), 5.0, 5.0).is_none());
    }
}