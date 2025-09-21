# Project Structure Guide

This guide explains how to organize code in the WorldKeeper project and provides templates for common patterns.

## Directory Structure

### Recommended Organization

```
worldkeeper/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library crate root (optional)
│   ├── prelude.rs           # Common imports
│   │
│   ├── systems/             # Game systems (business logic)
│   │   ├── mod.rs
│   │   ├── input.rs         # Input handling
│   │   ├── movement.rs      # Entity movement
│   │   ├── physics.rs       # Physics simulation
│   │   ├── combat.rs        # Combat mechanics
│   │   ├── ai.rs            # AI behavior
│   │   └── ui.rs            # User interface
│   │
│   ├── components/          # ECS components (data)
│   │   ├── mod.rs
│   │   ├── player.rs        # Player components
│   │   ├── physics.rs       # Physics components
│   │   ├── combat.rs        # Combat components
│   │   └── ui.rs            # UI components
│   │
│   ├── resources/           # Global game state
│   │   ├── mod.rs
│   │   ├── game_state.rs    # Game state resource
│   │   ├── config.rs        # Configuration
│   │   └── time.rs          # Time-related resources
│   │
│   ├── events/              # Custom events
│   │   ├── mod.rs
│   │   ├── input_events.rs  # Input events
│   │   ├── game_events.rs   # Game logic events
│   │   └── ui_events.rs     # UI events
│   │
│   ├── plugins/             # Bevy plugins
│   │   ├── mod.rs
│   │   ├── core_plugin.rs   # Core game plugin
│   │   ├── ui_plugin.rs     # UI plugin
│   │   └── debug_plugin.rs  # Debug tools
│   │
│   └── utils/               # Utility functions
│       ├── mod.rs
│       ├── math.rs          # Math utilities
│       ├── constants.rs     # Game constants
│       └── helpers.rs       # Helper functions
│
├── assets/                  # Game assets
│   ├── sprites/
│   ├── audio/
│   ├── fonts/
│   └── data/
│
├── docs/                    # Documentation
│   ├── architecture.md
│   └── project_structure.md
│
├── tests/                   # Integration tests
│   ├── common/
│   └── integration_tests.rs
│
├── examples/                # Example code
│   └── basic_setup.rs
│
├── Cargo.toml              # Dependencies and metadata
├── README.md               # Project overview
├── GAME_DESIGN.md          # Game design document
└── DEVELOPMENT.md          # Development guidelines
```

## Module Templates

### System Module Template

Create new systems following this pattern:

```rust
// src/systems/example_system.rs
use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::events::*;

/// Example system that demonstrates the standard pattern
pub fn example_system(
    // Commands for spawning/despawning entities
    mut commands: Commands,
    
    // Queries for accessing entities and components
    mut entity_query: Query<(&mut Transform, &ExampleComponent)>,
    
    // Resources for global state
    game_state: Res<GameState>,
    time: Res<Time>,
    
    // Events for communication
    mut example_events: EventWriter<ExampleEvent>,
    mut input_events: EventReader<InputEvent>,
) {
    // System logic here
    for (mut transform, example_comp) in &mut entity_query {
        // Process entities
    }
    
    // Handle events
    for event in input_events.read() {
        // Process input events
    }
    
    // Send events if needed
    example_events.send(ExampleEvent { data: 42 });
}

// System sets for ordering
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ExampleSystemSet {
    PreUpdate,
    Update,
    PostUpdate,
}
```

### Component Module Template

```rust
// src/components/example_component.rs
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Example component with documentation
#[derive(Component, Debug, Clone, PartialEq)]
pub struct ExampleComponent {
    pub value: f32,
    pub active: bool,
}

impl ExampleComponent {
    /// Create a new example component
    pub fn new(value: f32) -> Self {
        Self {
            value,
            active: true,
        }
    }
    
    /// Check if the component is active
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Default for ExampleComponent {
    fn default() -> Self {
        Self::new(0.0)
    }
}

// Marker components (no data)
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

// Bundle for related components
#[derive(Bundle)]
pub struct ExampleBundle {
    pub example: ExampleComponent,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for ExampleBundle {
    fn default() -> Self {
        Self {
            example: ExampleComponent::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}
```

### Resource Module Template

```rust
// src/resources/example_resource.rs
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Example resource for global state
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct ExampleResource {
    pub counter: u32,
    pub enabled: bool,
}

impl ExampleResource {
    /// Create a new resource
    pub fn new() -> Self {
        Self {
            counter: 0,
            enabled: true,
        }
    }
    
    /// Increment the counter
    pub fn increment(&mut self) {
        self.counter += 1;
    }
    
    /// Reset the resource
    pub fn reset(&mut self) {
        self.counter = 0;
        self.enabled = true;
    }
}

impl Default for ExampleResource {
    fn default() -> Self {
        Self::new()
    }
}
```

### Event Module Template

```rust
// src/events/example_events.rs
use bevy::prelude::*;

/// Example event for system communication
#[derive(Event, Debug, Clone)]
pub struct ExampleEvent {
    pub entity: Entity,
    pub value: f32,
}

/// Event for player actions
#[derive(Event, Debug, Clone)]
pub struct PlayerActionEvent {
    pub player: Entity,
    pub action: PlayerAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerAction {
    Move(Vec2),
    Attack,
    UseItem(ItemId),
}

/// Helper type for item IDs
pub type ItemId = u32;
```

### Plugin Module Template

```rust
// src/plugins/example_plugin.rs
use bevy::prelude::*;
use crate::systems::*;
use crate::components::*;
use crate::resources::*;
use crate::events::*;

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add resources
            .init_resource::<ExampleResource>()
            
            // Add events
            .add_event::<ExampleEvent>()
            
            // Add systems
            .add_systems(
                Startup,
                setup_example_system
            )
            .add_systems(
                Update,
                (
                    example_system,
                    handle_example_events,
                ).in_set(ExampleSystemSet::Update)
            )
            
            // Configure system sets
            .configure_sets(
                Update,
                (
                    ExampleSystemSet::PreUpdate,
                    ExampleSystemSet::Update,
                    ExampleSystemSet::PostUpdate,
                ).chain()
            );
    }
}

fn setup_example_system(
    mut commands: Commands,
    // other parameters
) {
    // Initialization logic
    info!("Example plugin initialized");
}

fn handle_example_events(
    mut example_events: EventReader<ExampleEvent>,
    // other parameters
) {
    for event in example_events.read() {
        info!("Received example event: {:?}", event);
    }
}
```

## Naming Conventions

### Files and Modules
- Use snake_case for file names
- Module names match file names
- Use descriptive names that indicate purpose

### Components
- Use noun phrases: `Health`, `PlayerInput`, `MovementSpeed`
- Use PascalCase for component names
- Keep components focused and single-purpose

### Systems
- Use verb phrases: `move_player`, `handle_collisions`, `update_ui`
- Use snake_case for system function names
- Group related systems with prefixes

### Resources
- Use noun phrases describing the data: `GameState`, `InputConfig`
- Use PascalCase for resource names
- Keep resources focused on specific global state

### Events
- Use past tense or noun phrases: `PlayerMoved`, `CollisionEvent`
- Use PascalCase for event names
- Include relevant context in event data

## Import Organization

### Standard Import Order

```rust
// Standard library imports
use std::collections::HashMap;
use std::time::Duration;

// Third-party crate imports
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use rand::prelude::*;

// Local module imports (crate-relative)
use crate::components::{Player, Health};
use crate::resources::GameState;
use crate::events::PlayerEvent;
use crate::utils::math::*;
```

### Prelude Module

Create a prelude module for commonly used imports:

```rust
// src/prelude.rs
pub use crate::components::*;
pub use crate::resources::*;
pub use crate::events::*;
pub use crate::systems::*;
pub use crate::utils::*;

// Re-export commonly used Bevy types
pub use bevy::prelude::*;
```

Then use it in modules:

```rust
// In any module
use crate::prelude::*;
```

## Testing Organization

### Unit Tests

```rust
// In the same file as the code being tested
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let component = ExampleComponent::new(42.0);
        assert_eq!(component.value, 42.0);
        assert!(component.is_active());
    }
}
```

### Integration Tests

```rust
// tests/integration_tests.rs
use worldkeeper::prelude::*;

#[test]
fn test_full_game_setup() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        ExamplePlugin,
    ));
    
    // Test that the app runs without panicking
    app.update();
}
```

### Test Utilities

```rust
// tests/common/mod.rs
use bevy::prelude::*;

pub fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}

pub fn spawn_test_entity(app: &mut App) -> Entity {
    let mut world = app.world_mut();
    world.spawn(Transform::default()).id()
}
```

## Documentation Standards

### Module Documentation

```rust
//! This module contains systems for handling player input and movement.
//! 
//! The main systems are:
//! - `handle_input`: Processes keyboard and mouse input
//! - `move_player`: Updates player position based on input
//! 
//! # Examples
//! 
//! ```rust
//! use worldkeeper::systems::input::*;
//! 
//! // Add systems to your app
//! app.add_systems(Update, (handle_input, move_player));
//! ```

use bevy::prelude::*;
```

### Function Documentation

```rust
/// Moves entities based on their velocity and the current time delta.
/// 
/// This system queries all entities with both `Transform` and `Velocity` components
/// and updates their position based on the velocity and frame time.
/// 
/// # Parameters
/// 
/// * `entity_query` - Query for entities with Transform and Velocity components
/// * `time` - Time resource for frame-independent movement
/// 
/// # Examples
/// 
/// ```rust
/// app.add_systems(Update, movement_system);
/// ```
pub fn movement_system(
    mut entity_query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    // Implementation
}
```

---

*This structure guide should be updated as the project grows and new patterns are established.*