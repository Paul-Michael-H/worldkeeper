# Development Guidelines

This document outlines coding standards, best practices, and development workflow for the WorldKeeper project.

## Table of Contents
1. [Code Organization](#code-organization)
2. [Bevy Best Practices](#bevy-best-practices)
3. [Rust Guidelines](#rust-guidelines)
4. [Development Workflow](#development-workflow)
5. [Performance Considerations](#performance-considerations)
6. [Testing Strategy](#testing-strategy)

## Code Organization

### Module Structure
```
src/
├── main.rs                 # App entry point and setup
├── lib.rs                  # Library root (if needed)
├── systems/                # Bevy systems
│   ├── mod.rs
│   ├── movement.rs
│   ├── input.rs
│   ├── ui.rs
│   └── game_logic.rs
├── components/             # ECS components
│   ├── mod.rs
│   ├── player.rs
│   ├── health.rs
│   └── physics.rs
├── resources/              # Global game state
│   ├── mod.rs
│   ├── game_state.rs
│   └── config.rs
├── plugins/                # Custom Bevy plugins
│   ├── mod.rs
│   ├── player_plugin.rs
│   └── ui_plugin.rs
├── events/                 # Custom events
│   ├── mod.rs
│   └── game_events.rs
├── utils/                  # Utility functions
│   ├── mod.rs
│   └── math.rs
└── prelude.rs              # Common imports
```

### File Naming
- Use snake_case for file and module names
- Use descriptive names that indicate purpose
- Group related functionality in modules

### Import Organization
```rust
// Standard library imports
use std::collections::HashMap;

// Third-party crates
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// Local modules
use crate::components::Player;
use crate::resources::GameState;
```

## Bevy Best Practices

### System Organization

#### System Naming
- Use descriptive verbs: `move_player`, `handle_collisions`, `update_ui`
- Group related systems with prefixes: `input_*`, `physics_*`, `ui_*`

#### System Parameters
```rust
// Good: Clear, focused system
fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // Implementation
}

// Avoid: Too many parameters, unclear purpose
fn update_everything(
    mut commands: Commands,
    mut query: Query<&mut Transform>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    // ... many more parameters
) {
    // Implementation
}
```

#### System Sets and Ordering
```rust
// Define system sets for clear execution order
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSystemSet {
    Input,
    Movement,
    Collision,
    UI,
}

// Configure system ordering
app.configure_sets(
    Update,
    (
        GameSystemSet::Input,
        GameSystemSet::Movement,
        GameSystemSet::Collision,
        GameSystemSet::UI,
    ).chain()
);
```

### Component Design

#### Component Guidelines
- Keep components small and focused
- Prefer composition over large, monolithic components
- Use marker components for simple tags

```rust
// Good: Small, focused components
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

// Avoid: Large, unfocused component
#[derive(Component)]
pub struct GameEntity {
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub damage: f32,
    pub is_player: bool,
    pub is_enemy: bool,
    // ... many more fields
}
```

### Resource Management

#### Resource Guidelines
- Use resources for global state only
- Prefer events for communication between systems
- Keep resources simple and focused

```rust
// Good: Focused resource
#[derive(Resource)]
pub struct GameState {
    pub current_level: u32,
    pub score: u32,
}

// Good: Configuration resource
#[derive(Resource)]
pub struct GameConfig {
    pub window_width: f32,
    pub window_height: f32,
    pub master_volume: f32,
}
```

### Event Handling

#### Custom Events
```rust
// Define clear, focused events
#[derive(Event)]
pub struct PlayerDied {
    pub player_id: Entity,
    pub cause: DeathCause,
}

#[derive(Event)]
pub struct ScoreChanged {
    pub old_score: u32,
    pub new_score: u32,
}

// Send events
fn check_player_health(
    mut player_query: Query<(Entity, &Health), With<Player>>,
    mut death_events: EventWriter<PlayerDied>,
) {
    for (entity, health) in &player_query {
        if health.current <= 0.0 {
            death_events.send(PlayerDied {
                player_id: entity,
                cause: DeathCause::NoHealth,
            });
        }
    }
}

// Receive events
fn handle_player_death(
    mut death_events: EventReader<PlayerDied>,
    // other parameters
) {
    for event in death_events.read() {
        // Handle the death
    }
}
```

## Rust Guidelines

### Code Style
- Follow official Rust style guidelines
- Use `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Use meaningful variable and function names

### Error Handling
```rust
// Prefer Result types for fallible operations
fn load_config(path: &str) -> Result<Config, ConfigError> {
    // Implementation
}

// Use ? operator for error propagation
fn game_setup() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config("config.toml")?;
    let assets = load_assets(&config.asset_path)?;
    Ok(())
}
```

### Documentation
```rust
/// Moves the player based on input
/// 
/// # Arguments
/// * `player_query` - Query for player entities with Transform
/// * `input` - Keyboard input resource
/// * `time` - Time resource for frame-independent movement
fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // Implementation
}
```

## Development Workflow

### Git Workflow
1. Create feature branches: `feature/player-movement`
2. Make small, focused commits
3. Write descriptive commit messages
4. Test before committing
5. Rebase before merging to main

### Commit Messages
```
feat: add player movement system
fix: resolve collision detection bug
docs: update API documentation
refactor: simplify input handling
test: add unit tests for health system
```

### Testing
- Write tests for core game logic
- Test components and systems in isolation
- Use integration tests for system interactions

### Code Review
- Review for Bevy best practices
- Check for performance implications
- Ensure proper error handling
- Verify documentation is up to date

## Performance Considerations

### Query Optimization
```rust
// Good: Specific queries
fn update_enemies(
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    // Implementation
}

// Avoid: Overly broad queries
fn update_entities(
    mut all_query: Query<&mut Transform>,
) {
    // Implementation that filters manually
}
```

### System Performance
- Use `Changed<T>` filters for change detection
- Batch operations when possible
- Profile regularly during development
- Consider parallel systems for independent operations

### Memory Management
- Reuse resources when possible
- Clean up unused entities promptly
- Use object pools for frequently created/destroyed entities

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_component() {
        let health = Health::new(100.0);
        assert_eq!(health.current, 100.0);
        assert_eq!(health.max, 100.0);
    }
}
```

### Integration Tests
- Test system interactions
- Verify event handling
- Test plugin functionality

### Manual Testing
- Regular playtesting
- Performance testing
- Cross-platform testing

## Tools and Setup

### Recommended VSCode Extensions
- `rust-analyzer`: Rust language support
- `CodeLLDB`: Debugging support
- `Even Better TOML`: TOML file support

### Useful Commands
```bash
# Development
cargo run --features dev        # Fast development builds
cargo check                     # Quick syntax checking
cargo clippy                    # Linting
cargo fmt                       # Code formatting

# Testing
cargo test                      # Run all tests
cargo test --lib               # Run library tests only
cargo test integration_tests   # Run integration tests

# Release
cargo build --release          # Optimized build
cargo run --release            # Run optimized version
```

---

*This document should be updated as the project evolves and new patterns emerge.*