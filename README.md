# Mithya Engine
A game engine built with Rust and ECS architecture.
Intent of this project is just to learn Rust language and have fun while doing it and for me having fun with code = making games.
As I get better at Rust, I will interate on all core systems upgrading them to follow better patterns and make them faster.

## Core Features
- **Entity Component System (ECS)**
  - Component-based architecture
  - Efficient entity queries
  - Archetype-based storage
- **Graphics**
  - SDL2 + OpenGL pipeline
  - Material system
  - Basic mesh rendering (Triangle and Quad)
  - Texture loading
- **Physics**
  - Simple 2d Collision detection with circle and box colliders
  - Simple rigid body dynamics
- **Core Systems**
  - Input handling
  - Player controls
  - Transform hierarchy

## Project Structure
```
mithya_engine/           # Core engine library
├── src/
│   ├── core/           # ECS and entity management
│   ├── engine/         # Core engine systems
│   ├── physics/        # Physics and collision
│   ├── rendering/      # Graphics systems
│   └── player/         # Player components and systems
│
mithya_sandbox/         # Demo game project
└── src/
    └── main.rs        # Example game implementation
```

## Quick Start
```powershell
# Install SDL2 on Windows using vcpkg
vcpkg install sdl2:x64-windows

# Build and run the sandbox demo
cd mithya_sandbox
cargo run
```

## Requirements
- Rust 1.70+
- SDL2 development libraries
- OpenGL 4.1+
