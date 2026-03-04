# Mithya Engine

A game engine built with Rust and ECS architecture. Intent is to learn Rust by building real systems and iterating on them as understanding improves.

---

## Architecture

### ECS
Entities are plain `u32` IDs. Components are data structs attached to entities. Systems operate on entities that have specific components. No inheritance — behaviour comes from component composition.

**Why:** Rust's borrow checker makes traditional OOP inheritance painful. ECS works naturally with Rust — systems take what they need by component type, ownership is clear, and there are no shared mutable base classes.

### Event / Action Pipeline
Systems communicate through two channels:

- **Events** — facts about what happened. Pushed from `update()`, broadcast to all interested listeners at end of frame.
- **Actions** — deferred world mutations. Pushed from event listeners, executed after all systems finish updating.

```
update() → push events
on_events() → hear events → push actions  
execute_all() → actions mutate world
```

**Why:** Systems can't hold mutable references to each other — Rust won't allow it. Events and actions solve this cleanly. A system signals what happened via an event without needing a reference to whoever cares. The receiver pushes an action without needing to mutate the world mid-frame. Mutation happens at a safe, predictable point after all systems have finished reading. This also makes systems fully decoupled — an audio system can react to `BrickDestroyedEvent` without `BrickBreakerSystem` knowing audio exists.

### Systems
All systems implement `System` (`initialize`, `update`, `render`). Systems that care about events implement `EngineEventListener` and declare which event types they want — only relevant systems are notified, no polling.

**Why:** The listener pattern avoids systems needing to scan the full event queue every frame. Each system declares its interests upfront via `TypeId` — the engine only calls `on_events` when a matching event exists.

### Rendering
Built on **wgpu** (Vulkan/DX12/Metal/WebGPU). Shaders in WGSL. Pipeline state — blend mode, depth, vertex layout — is compiled upfront, not toggled at runtime. Uniforms passed via typed bind groups.

**Why:** OpenGL uses a global state machine with implicit context. wgpu is explicit — no hidden state, no unsafe, resource lifetimes tracked by Rust's ownership system. Pipeline compilation upfront means the GPU has no surprises at render time. The explicit bind group model also maps cleanly to Rust's type system unlike OpenGL's string-based uniform locations.

`RenderingSystem` is the only system that touches the GPU directly. Other systems visualise debug info by spawning entities with `Render` components — `RenderingSystem` picks them up automatically.

### Assets
Asset root is configured per-game via `EngineConfig`. Games call `load_texture_for_material()` in `initialize()` — the engine has no knowledge of game assets. `RenderingSystem` exposes `load_assets()` as a closure so games receive `device` and `queue` without those leaking into the public API.

**Why:** Early versions had hardcoded texture paths and material names inside the engine. This broke as soon as a second game was added. The closure pattern keeps wgpu internals contained inside `RenderingSystem` while giving games full control over what they load.

---

## Stack

`wgpu` · `winit` · `glam` · `serde` · `bytemuck` · `image` · `thiserror`

---

## Sample Game — Brickbreaker

Built to exercise all engine systems. Ball physics, paddle deflection, brick destruction, scoring, lives, win/lose/reset.

Controls: `Space` launch · `A/D` or arrows move · `Enter` reset

---

## Roadmap

- [ ] Controller / Pawn pattern — player and AI controllers possessing pawns
- [ ] AI controller — world-aware paddle driver
- [ ] Self-learning controller — trains to play autonomously
- [ ] UI system — egui-wgpu replacing current stub
- [ ] Camera system — replace hardcoded projection
- [ ] 3D rendering