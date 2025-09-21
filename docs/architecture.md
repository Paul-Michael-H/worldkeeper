# Architecture Overview

This document provides a high-level overview of the WorldKeeper game architecture and how the various systems work together.

## Architecture Philosophy

WorldKeeper follows Entity Component System (ECS) architecture principles provided by the Bevy game engine. This approach promotes:

- **Modularity**: Systems are independent and focused
- **Performance**: Data-oriented design for cache efficiency  
- **Flexibility**: Easy to add, remove, or modify game features
- **Testability**: Systems can be tested in isolation

## System Architecture

### Core Architecture Layers

```
┌─────────────────────────────────────────┐
│                 Game Logic              │
│            (Systems & Events)           │
├─────────────────────────────────────────┤
│              ECS Framework              │
│         (Entities, Components)          │
├─────────────────────────────────────────┤
│               Bevy Engine               │
│    (Rendering, Audio, Input, Time)      │
├─────────────────────────────────────────┤
│              Platform Layer             │
│        (Windows, macOS, Linux)          │
└─────────────────────────────────────────┘
```

### Entity Component System (ECS)

#### Entities
Entities are unique identifiers that represent game objects. Examples:
- Player character
- Enemy entities
- Environment objects
- UI elements

#### Components
Components are data structures attached to entities:

```rust
// Position and movement
Transform, Velocity, Acceleration

// Game-specific
Player, Enemy, Health, Weapon

// Rendering
Sprite, Mesh, Material

// Physics
Collider, RigidBody
```

#### Systems
Systems contain the game logic and operate on entities with specific components:

```rust
// Core systems
movement_system()       // Updates Transform based on Velocity
collision_system()      // Handles entity collisions
input_system()         // Processes player input
render_system()        // Renders entities to screen

// Game-specific systems
combat_system()        // Handles damage and combat
ai_system()           // Controls enemy behavior
ui_system()           // Updates user interface
```

## Module Organization

### Source Code Structure

```
src/
├── main.rs                 # Application entry point
├── systems/                # Game systems
│   ├── mod.rs             # System module exports
│   ├── input.rs           # Input handling
│   ├── movement.rs        # Entity movement
│   ├── collision.rs       # Collision detection
│   ├── combat.rs          # Combat mechanics
│   ├── ai.rs              # AI behavior
│   └── ui.rs              # User interface
├── components/             # ECS components
│   ├── mod.rs             # Component exports
│   ├── player.rs          # Player-specific components
│   ├── physics.rs         # Physics components
│   ├── combat.rs          # Combat-related components
│   └── ui.rs              # UI components
├── resources/              # Global game state
│   ├── mod.rs             # Resource exports
│   ├── game_state.rs      # Current game state
│   ├── config.rs          # Game configuration
│   └── assets.rs          # Asset management
├── events/                 # Custom game events
│   ├── mod.rs             # Event exports
│   ├── combat_events.rs   # Combat-related events
│   └── ui_events.rs       # UI interaction events
├── plugins/                # Bevy plugins
│   ├── mod.rs             # Plugin exports
│   ├── game_plugin.rs     # Main game plugin
│   ├── ui_plugin.rs       # UI plugin
│   └── debug_plugin.rs    # Development tools
└── utils/                  # Utility functions
    ├── mod.rs             # Utility exports
    ├── math.rs            # Mathematical utilities
    └── constants.rs       # Game constants
```

## System Execution Order

### Update Schedule

Bevy systems run in a specific order during each frame:

```
Frame Start
├── Pre-Update
│   ├── Input Collection
│   └── Event Processing
├── Update
│   ├── Input Systems
│   ├── Game Logic Systems
│   ├── Physics Systems
│   └── UI Systems
├── Post-Update
│   ├── Cleanup Systems
│   └── State Updates
└── Render
    ├── Transform Updates
    ├── Visibility Culling
    └── Draw Commands
Frame End
```

### System Sets

Systems are organized into sets for predictable execution order:

```rust
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSystemSet {
    Input,      // Process input first
    Logic,      // Game logic updates
    Physics,    // Physics simulation
    Audio,      // Audio updates
    UI,         // UI updates last
}
```

## Data Flow

### Input → Logic → Rendering Pipeline

```
User Input
    ↓
Input Systems (keyboard, mouse)
    ↓
Game Logic Systems (movement, combat, AI)
    ↓
Physics Systems (collision, forces)
    ↓
State Update Systems (health, score, game state)
    ↓
Audio Systems (sound effects, music)
    ↓
UI Systems (HUD, menus)
    ↓
Rendering Systems (sprites, effects)
    ↓
Frame Display
```

### Event-Driven Communication

Systems communicate through Bevy's event system:

```rust
// Event definition
#[derive(Event)]
pub struct PlayerDamaged {
    pub entity: Entity,
    pub damage: f32,
}

// Event sender
fn combat_system(
    mut damage_events: EventWriter<PlayerDamaged>,
    // ... other parameters
) {
    // Send event when damage occurs
    damage_events.send(PlayerDamaged {
        entity: player_entity,
        damage: 10.0,
    });
}

// Event receiver
fn ui_system(
    mut damage_events: EventReader<PlayerDamaged>,
    // ... other parameters
) {
    for event in damage_events.read() {
        // Update UI to show damage
    }
}
```

## Asset Management

### Asset Organization

```
assets/
├── sprites/               # 2D graphics
│   ├── characters/
│   ├── environment/
│   └── ui/
├── audio/                 # Sound files
│   ├── music/
│   ├── sfx/
│   └── voice/
├── fonts/                 # Text rendering
├── shaders/               # Custom shaders
└── data/                  # Configuration files
    ├── levels/
    └── config.toml
```

### Asset Loading Strategy

- **Startup Loading**: Essential assets loaded during initialization
- **Lazy Loading**: Level-specific assets loaded as needed
- **Streaming**: Large assets loaded in background
- **Caching**: Reuse loaded assets across scenes

## Performance Considerations

### ECS Optimization

1. **Query Efficiency**: Use specific component filters
2. **Change Detection**: Only process modified entities
3. **Parallel Systems**: Run independent systems in parallel
4. **Memory Layout**: Group related components together

### Rendering Optimization

1. **Sprite Batching**: Combine similar sprites in single draw call
2. **Culling**: Don't render off-screen entities
3. **LOD**: Use lower detail for distant objects
4. **Texture Atlasing**: Combine small textures

## Development Patterns

### Plugin Architecture

Break functionality into focused plugins:

```rust
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            movement_system,
            collision_system,
        ));
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_system);
    }
}
```

### State Management

Use Bevy states for game flow:

```rust
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}
```

## Testing Strategy

### Unit Testing
- Test individual systems with mock data
- Verify component behavior
- Test utility functions

### Integration Testing
- Test system interactions
- Verify event handling
- Test complete gameplay scenarios

### Performance Testing
- Profile system execution times
- Monitor memory usage
- Test with large entity counts

---

*This architecture overview should be updated as the game develops and new patterns emerge.*