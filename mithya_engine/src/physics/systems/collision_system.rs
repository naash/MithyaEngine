// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{any::Any, collections::HashSet};

use glam::{Vec2, Vec3Swizzles};

use crate::{
    core::{EngineEvent, Transform},
    engine::{system::{System, SystemRenderContext, SystemUpdateContext}, World}, 
    physics::components::{Collider, ColliderShape, RigidBody},
};

#[derive(Debug, Clone)]
pub struct CollisionEvent {
    pub entity_a: u32,
    pub entity_b: u32,
}

impl EngineEvent for CollisionEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {
        let collider_entities = update_context.world.entity_manager
            .query_two_components::<Transform, Collider>();

        let mut colliding_entities = HashSet::new();

        // Check all pairs of entities
        for i in 0..collider_entities.len() {
            for j in i + 1..collider_entities.len() {
                let entity_a = collider_entities[i];
                let entity_b = collider_entities[j];

                if let (Some(transform_a), Some(collider_a)) = (
                    update_context.world.entity_manager.get_component::<Transform>(entity_a),
                    update_context.world.entity_manager.get_component::<Collider>(entity_a)
                ) {
                    if let (Some(transform_b), Some(collider_b)) = (
                        update_context.world.entity_manager.get_component::<Transform>(entity_b),
                        update_context.world.entity_manager.get_component::<Collider>(entity_b)
                    ) {
                        if check_collision(transform_a, collider_a, transform_b, collider_b) {

                            //If collision is detected we resolve collision in 2 ways
                            //change position 
                            //update velocity 
                            resolve_collision(update_context.world, entity_a, entity_b);
                            
                            // Track both entities as colliding
                            colliding_entities.insert(entity_a);
                            colliding_entities.insert(entity_b);

                            //Log collision event
                            update_context.events.push(CollisionEvent{entity_a, entity_b});
                        }
                    }
                }
            }
        }

        // Update collision data 
        for &entity in &collider_entities {
            if let Some(collider) = update_context.world.entity_manager.get_component_mut::<Collider>(entity) {
                collider.is_colliding = colliding_entities.contains(&entity);
            }
        }
    }

    fn render(&mut self, _render_context: &mut SystemRenderContext) {
        // Collision system doesn't render anything, might render debug stuff?
    }
}

fn check_collision(
    transform_a: &Transform,
    collider_a: &Collider,
    transform_b: &Transform,
    collider_b: &Collider,
) -> bool {
    let pos_a = transform_a.position + collider_a.offset;
    let pos_b = transform_b.position + collider_b.offset;

    let scale_a = transform_a.scale;
    let scale_b = transform_b.scale;

    //Collision logic is only for 2D
    match (&collider_a.shape, &collider_b.shape) {
        (ColliderShape::Circle { radius: r1 }, ColliderShape::Circle { radius: r2 }) => {
            
            let scaled_r1 = r1 * scale_a.x.max(scale_a.y);
            let scaled_r2 = r2 * scale_b.x.max(scale_b.y);
          
            let distance = (pos_a - pos_b).length();
            distance < scaled_r1 + scaled_r2
        }
        (ColliderShape::Box { width: w1, height: h1 }, ColliderShape::Box { width: w2, height: h2 }) => {
            
            let scaled_w1 = w1 * scale_a.x;
            let scaled_h1 = h1 * scale_a.y;
            let scaled_w2 = w2 * scale_b.x;
            let scaled_h2 = h2 * scale_b.y;
            
            let half_w1 = scaled_w1 / 2.0;
            let half_h1 = scaled_h1 / 2.0;
            let half_w2 = scaled_w2 / 2.0;
            let half_h2 = scaled_h2 / 2.0;

            pos_a.x - half_w1 < pos_b.x + half_w2 &&
            pos_a.x + half_w1 > pos_b.x - half_w2 &&
            pos_a.y - half_h1 < pos_b.y + half_h2 &&
            pos_a.y + half_h1 > pos_b.y - half_h2
        }
        // Circle-Box collision (circle A, box B)
        (ColliderShape::Circle { radius }, ColliderShape::Box { width, height }) => {

            let scaled_radius = radius * scale_a.x.max(scale_a.y);
            let scaled_width = width * scale_b.x;
            let scaled_height = height * scale_b.y;

            check_circle_box_collision(pos_a.xy(), scaled_radius, pos_b.xy(), scaled_width, scaled_height)
        }
        // Box-Circle collision (box A, circle B)
        (ColliderShape::Box { width, height }, ColliderShape::Circle { radius }) => {

            let scaled_radius = radius * scale_b.x.max(scale_b.y);
            let scaled_width = width * scale_a.x;
            let scaled_height = height * scale_a.y;

            check_circle_box_collision(pos_b.xy(), scaled_radius, pos_a.xy(), scaled_width, scaled_height)
        }
    }
}

fn check_circle_box_collision(
    circle_pos: Vec2,
    circle_radius: f32,
    box_pos: Vec2,
    box_width: f32,
    box_height: f32,
) -> bool {
    let closest_point = closest_point_on_box_to_circle(box_pos, circle_pos, box_width, box_height);
    let distance = (circle_pos - closest_point).length();
    distance < circle_radius
}

fn resolve_collision(world: &mut World, entity_a: u32, entity_b: u32) {

    // Get the positions and colliders again to calculate separation
    let (separation_distance, collision_normal) = {
        let transform_a = world.entity_manager.get_component::<Transform>(entity_a).unwrap();
        let transform_b = world.entity_manager.get_component::<Transform>(entity_b).unwrap();
        let collider_a = world.entity_manager.get_component::<Collider>(entity_a).unwrap();
        let collider_b = world.entity_manager.get_component::<Collider>(entity_b).unwrap();
        
        let pos_a = transform_a.position + collider_a.offset;
        let pos_b = transform_b.position + collider_b.offset;
        
        // Calculate collision normal and separation distance
        let (normal, separation) = calculate_collision_response(pos_a.xy(), pos_b.xy(), &collider_a, &collider_b, &transform_a, &transform_b);

        (separation, normal)
    };
    
    // Skip if separation distance is invalid
    if separation_distance <= 0.001 {
        println!("Invalid separation distance, skipping collision response");
        return;
    }
    
    // Check which objects have rigidbodies (only move objects with rigidbodies)
    let has_rigidbody_a = world.entity_manager.get_component::<RigidBody>(entity_a).is_some();
    let has_rigidbody_b = world.entity_manager.get_component::<RigidBody>(entity_b).is_some();

    // Use the collision normal for separation  
    // Normal points from A toward B, so:
    // - Move A in +normal direction (away from B)
    // - Move B in -normal direction (away from A)
    // Add offset to positions
    if has_rigidbody_a && has_rigidbody_b {
        // Both have rigidbodies, split the movement
        let half_move = separation_distance / 2.0;
        
        if let Some(transform_a) = world.entity_manager.get_component_mut::<Transform>(entity_a) {
            transform_a.position.x += collision_normal.x * half_move;
            transform_a.position.y += collision_normal.y * half_move;
            //println!("Moved entity {} by {:?}", entity_a, Vec2::new(collision_normal.x * half_move, collision_normal.y * half_move));
        }
        
        if let Some(transform_b) = world.entity_manager.get_component_mut::<Transform>(entity_b) {
            transform_b.position.x -= collision_normal.x * half_move;
            transform_b.position.y -= collision_normal.y * half_move;
            //println!("Moved entity {} by {:?}", entity_b, Vec2::new(-collision_normal.x * half_move, -collision_normal.y * half_move));
        }
    } else if has_rigidbody_a {
        // Only A has rigidbody, move A away from B (in +normal direction)
        if let Some(transform_a) = world.entity_manager.get_component_mut::<Transform>(entity_a) {
            transform_a.position.x += collision_normal.x * separation_distance;
            transform_a.position.y += collision_normal.y * separation_distance;
            //println!("Moved entity {} by {:?}", entity_a, Vec2::new(collision_normal.x * separation_distance, collision_normal.y * separation_distance));
        }
    } else if has_rigidbody_b {
        // Only B has rigidbody, move B away from A (in -normal direction)
        if let Some(transform_b) = world.entity_manager.get_component_mut::<Transform>(entity_b) {
            transform_b.position.x -= collision_normal.x * separation_distance;
            transform_b.position.y -= collision_normal.y * separation_distance;
            //println!("Moved entity {} by {:?}", entity_b, Vec2::new(-collision_normal.x * separation_distance, -collision_normal.y * separation_distance));
        }
    }

    if separation_distance > 0.5 {
        // Dampen velocity to prevent jittering
        if let Some(rigid_body_a) = world.entity_manager.get_component_mut::<RigidBody>(entity_a) {
            rigid_body_a.velocity *= 0.95;
        }
        if let Some(rigid_body_b) = world.entity_manager.get_component_mut::<RigidBody>(entity_b) {
            rigid_body_b.velocity *= 0.95;
        }
    }

    //Reflect velocity if bounce exists
    if has_rigidbody_a {
        if let Some(rigid_body_a) = world.entity_manager.get_component_mut::<RigidBody>(entity_a) {

            if !rigid_body_a.is_kinematic {
                
                let velocity = Vec2::new(rigid_body_a.velocity.x, rigid_body_a.velocity.y);
                let vel_along_normal = velocity.dot(collision_normal);
            
                // If velocity is opposite to normal (moving into B), reflect
                if vel_along_normal < 0.0 {
                // Reflect: v' = v - 2(v·n)n
                let reflected = velocity - 2.0 * vel_along_normal * collision_normal;
                rigid_body_a.velocity.x = reflected.x * rigid_body_a.bounce;
                rigid_body_a.velocity.y = reflected.y * rigid_body_a.bounce;
                }
            }
        }
    }
    
    if has_rigidbody_b {
        if let Some(rigid_body_b) = world.entity_manager.get_component_mut::<RigidBody>(entity_b) {

            if !rigid_body_b.is_kinematic {
                
                let velocity = Vec2::new(rigid_body_b.velocity.x, rigid_body_b.velocity.y);
                let vel_along_normal = velocity.dot(collision_normal);
            
                // If velocity is opposite to normal (moving into B), reflect
                if vel_along_normal < 0.0 {
                // Reflect: v' = v - 2(v·n)n
                let reflected = velocity - 2.0 * vel_along_normal * collision_normal;
                rigid_body_b.velocity.x = reflected.x * rigid_body_b.bounce;
                rigid_body_b.velocity.y = reflected.y * rigid_body_b.bounce;
                }
            }
        }
    }
}

// Combined function to calculate both collision normal and separation distance
// This ensures consistency between the two calculations  
// Normal always points from A toward B (direction to separate A from B)
fn calculate_collision_response(
    pos_a: Vec2,
    pos_b: Vec2,
    collider_a: &Collider,
    collider_b: &Collider,
    transform_a: &Transform,
    transform_b: &Transform
) -> (Vec2, f32) {
    match (&collider_a.shape, &collider_b.shape) {
        (ColliderShape::Circle { radius: r1 }, ColliderShape::Circle { radius: r2 }) => {
            
            let scaled_r1 = r1 * transform_a.scale.x.max(transform_a.scale.y);
            let scaled_r2 = r2 * transform_b.scale.x.max(transform_b.scale.y);

            let diff = pos_a - pos_b;  // Vector from B to A
            let current_distance = diff.length();
            let min_distance = scaled_r1 + scaled_r2;
            
            let normal = if current_distance > 0.0 {
                diff / current_distance // Normalize - points from B to A
            } else {
                Vec2::new(1.0, 0.0) // Default direction
            };
            
            let separation = (min_distance - current_distance + 0.001).max(0.0); // Much smaller buffer
            (normal, separation)
        }
        (ColliderShape::Box { width: w1, height: h1 }, ColliderShape::Box { width: w2, height: h2 }) => {
            
            let scaled_w1 = w1 * transform_a.scale.x;
            let scaled_h1 = h1 * transform_a.scale.y;
            let scaled_w2 = w2 * transform_b.scale.x;
            let scaled_h2 = h2 * transform_b.scale.y;
            
            let diff = pos_a - pos_b;  // Vector from B to A
            let overlap_x = (scaled_w1 + scaled_w2) / 2.0 - diff.x.abs();
            let overlap_y = (scaled_h1 + scaled_h2) / 2.0 - diff.y.abs();
            
            let normal = if overlap_x < overlap_y {
                Vec2::new(if diff.x > 0.0 { 1.0 } else { -1.0 }, 0.0)
            } else {
                Vec2::new(0.0, if diff.y > 0.0 { 1.0 } else { -1.0 })
            };
            
            let separation = (overlap_x.min(overlap_y) + 0.001).max(0.0); // Much smaller buffer
            (normal, separation)
        }
        (ColliderShape::Circle { radius }, ColliderShape::Box { width, height }) => {
            
            let scaled_radius = radius * transform_a.scale.x.max(transform_a.scale.y);
            let scaled_width = width * transform_b.scale.x;
            let scaled_height = height * transform_b.scale.y;
            
            // Circle A vs Box B - normal should point from box toward circle
            calculate_circle_box_response(pos_a, scaled_radius, pos_b, scaled_width, scaled_height)
        }
        (ColliderShape::Box { width, height }, ColliderShape::Circle { radius }) => {
            
            let scaled_radius = radius * transform_b.scale.x.max(transform_b.scale.y);
            let scaled_width = width * transform_a.scale.x;
            let scaled_height = height * transform_a.scale.y;
            
            // Box A vs Circle B - normal should point from circle toward box
            let (normal, separation) = calculate_circle_box_response(pos_b, scaled_radius, pos_a, scaled_width, scaled_height);
            (-normal, separation) // Flip the normal to point from A to B
        }
    }
}

fn calculate_circle_box_response(
    circle_pos: Vec2,
    circle_radius: f32,
    box_pos: Vec2,
    box_width: f32,
    box_height: f32,
) -> (Vec2, f32) {
    let half_width = box_width / 2.0;
    let half_height = box_height / 2.0;
    
    // Check if circle center is inside the box
    let box_min = Vec2::new(box_pos.x - half_width, box_pos.y - half_height);
    let box_max = Vec2::new(box_pos.x + half_width, box_pos.y + half_height);
    
    let circle_inside_box = circle_pos.x >= box_min.x && circle_pos.x <= box_max.x &&
                           circle_pos.y >= box_min.y && circle_pos.y <= box_max.y;
    
    if circle_inside_box {
        // Circle is inside the box, find the shortest path out
        let dist_to_left = circle_pos.x - box_min.x;
        let dist_to_right = box_max.x - circle_pos.x;
        let dist_to_bottom = circle_pos.y - box_min.y;
        let dist_to_top = box_max.y - circle_pos.y;
        
        let min_dist = dist_to_left.min(dist_to_right).min(dist_to_bottom).min(dist_to_top);
        
        let normal = if min_dist == dist_to_left {
            Vec2::new(-1.0, 0.0)  // Push left
        } else if min_dist == dist_to_right {
            Vec2::new(1.0, 0.0)   // Push right
        } else if min_dist == dist_to_bottom {
            Vec2::new(0.0, -1.0)  // Push down
        } else {
            Vec2::new(0.0, 1.0)   // Push up
        };
        
        let separation = circle_radius + min_dist;
        (normal, separation)
    } else {
        // Circle is outside the box, use closest point method
        let closest_point = closest_point_on_box_to_circle(box_pos, circle_pos, box_width, box_height);
        
        // Vector from closest point to circle center
        let to_circle = circle_pos - closest_point;
        let distance = to_circle.length();
        
        if distance > 0.0001 {
            // Normal points from closest point to circle center
            let normal = to_circle / distance;
            let separation = (circle_radius - distance).max(0.0);
            (normal, separation)
        } else {
            // Circle center is exactly on the box surface, push away from box center
            let to_circle = circle_pos - box_pos;
            let normal = if to_circle.length() > 0.0001 {
                to_circle / to_circle.length()
            } else {
                Vec2::new(1.0, 0.0) // Default direction
            };
            (normal, circle_radius)
        }
    }
}

fn closest_point_on_box_to_circle(box_pos: Vec2, circle_pos: Vec2, box_width: f32, box_height: f32) -> Vec2 {
    let half_width = box_width / 2.0;
    let half_height = box_height / 2.0;
    
    // Clamp the circle position to the box bounds
    Vec2::new(
        circle_pos.x.clamp(box_pos.x - half_width, box_pos.x + half_width),
        circle_pos.y.clamp(box_pos.y - half_height, box_pos.y + half_height)
    )
}