# Mithya Engine

A game engine built from scratch in Rust, driven by a single goal: learn Rust by building something real, functional.

---

## Architecture

### ECS
Entities are plain `u32` IDs. Components are data structs. Systems operate on entities that have specific components. No inheritance — behaviour comes from composition.

### Event / Action Pipeline
Systems communicate through two channels — events (facts about what happened) and actions (deferred world mutations). Events are pushed from `update()`, broadcast to listeners, which push actions that execute after all systems finish.

```
update() → push events
on_events(events, actions, world) → push actions
execute_all() → mutate world
```

`on_events` receives read-only `&World` access so listeners can query component state when deciding which actions to push — without being able to mutate mid-frame.

### Resources
Global data that doesn't belong to any entity lives in a typed `Resources` store keyed by type rather than by name.

```rust
world.resources.insert(InputMapping::new());
world.resources.insert(PhysicsConfig::default());
world.resources.insert(Time::default());

world.resources.get::<InputMapping>()
world.resources.get_mut::<Time>()
```

This keeps `World` lean — only `EntityManager` and `AssetManager` live directly on it. Everything else is a resource. Adding new global data never touches the `World` struct.

### Input
Named input actions bound to physical keys via `InputMapping` resource. `InputSystem` translates raw key events into `InputActionEvent` — systems respond to actions, not keycodes. Rebinding is a config change, not a code change. `InputActionMode::Continuous` fires every frame while held, `InputActionMode::OneShot` fires once on press.

### Camera
A `Camera` component on any entity drives the view. `RenderingSystem` finds the active camera each frame and computes view and projection matrices from it — no hardcoded values. `size` controls the half-height in world units, aspect ratio is derived from the window dimensions automatically.

```rust
EntityBuilder::new(&mut world.entity_manager)
    .with(Transform::default())
    .with(Camera::new(20.0))  // 20 world units half-height
    .build();
```

### Collision Layers
Colliders have a `layer` (what they are) and a `mask` (what they collide with), both `u32` bitmasks — 32 layers available. Two entities only check collision if their layers and masks intersect, eliminating spurious wall-wall checks.

```rust
pub const LAYER_WALL: u32   = 0b0001;
pub const LAYER_BALL: u32   = 0b0010;
pub const LAYER_PADDLE: u32 = 0b0100;
pub const LAYER_BRICK: u32  = 0b1000;

//example Wall only collides with ball — never with other walls
Collider { layer: LAYER_WALL, mask: LAYER_BALL, .. }
```

### Rendering
Built on **wgpu** (Vulkan/DX12/Metal/WebGPU). Shaders in WGSL. `RenderingSystem` is stored separately from the system list — it owns all wgpu state and is called directly from the engine loop. UI via **egui** rendered on top of the main pass — games register a draw closure that executes every frame with read access to world state.

### Physics
Velocity, acceleration, drag, gravity, bounce, max speed. Collision resolution uses relative velocity reflection to preserve ball speed regardless of what it hits.

---

## What I Learned

This project was deliberately chosen as a Rust learning vehicle because game engines stress-test every part of the language. As a C++ programmer, learning Rust was interesting and its constraints forced me to architect the engine in a more robust and safer way.

**Ownership ended null pointer bugs entirely.** Every `get_component` returns `Option<T>`. The compiler forces handling of the missing case — there is no way to accidentally dereference a missing component. This class of bug simply doesn't exist.

**The borrow checker is a systems architecture tool.** Early versions of the engine had systems trying to hold references to each other but the compiler rejected all of it. The event/action pipeline wasn't a design choice, it was the solution the borrow checker forced. A system can't hold a `&mut World` while another system reads it, so mutations are deferred to a point where no borrows are active. The architecture is better because of the constraint.

**`Option<T>` and `Result<T,E>` replace entire categories of runtime errors.** No exceptions, no null checks, no undefined behaviour. The compiler won't let you use a value that might not exist without explicitly handling both cases. Writing `expect("message")` instead of `unwrap()` forced me to justify every assumption the code makes.

**Traits compose where inheritance breaks.** `System`, `EngineEventListener`, `Component`, `EngineAction` are all traits. A type can implement any combination. No base classes, no diamond problem, no vtable surprises. The ECS architecture maps naturally to this — entities are composed of components, behaviour comes from which traits those components' systems implement.

**Move semantics make resource management explicit.** Passing the wgpu `Device` to a function transfers ownership — you can't accidentally use it from two places. The GPU resource lifetime is enforced by the type system, not by discipline.

---

## Stack

`wgpu` · `winit` · `egui` · `glam` · `serde` · `bytemuck` · `image` · `thiserror`

