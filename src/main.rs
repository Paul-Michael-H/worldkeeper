use bevy::prelude::*;

// Module declarations
mod components;
mod resources;
mod systems;
mod plugins;

// Use our modules
use resources::GameState;
use plugins::{MenuPlugin, NewWorldkeeperPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "WorldKeeper".into(),
                resolution: (1024.0, 768.0).into(),
                ..default()
            }),
            ..default()
        }))
        // Add our custom plugins
        .add_plugins((
            MenuPlugin,
            NewWorldkeeperPlugin,
        ))
        // Setup systems that only run in InGame state
        .add_systems(OnEnter(GameState::InGame), setup_game)
        .add_systems(
            Update,
            (
                handle_input,
                move_camera,
            ).run_if(in_state(GameState::InGame))
        )
        .add_systems(OnExit(GameState::InGame), cleanup_game)
        .run();
}

/// Setup the game world when entering InGame state
fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Setting up game world");
    
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn a simple colored rectangle as a placeholder
    commands.spawn(ColorMesh2dBundle {
        mesh: meshes.add(Rectangle::new(100.0, 100.0)).into(),
        material: materials.add(Color::srgb(0.8, 0.2, 0.3)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });

    info!("Game world initialized!");
}

/// Cleanup game world when exiting InGame state
fn cleanup_game(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Camera>, With<Handle<Mesh>>)>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    info!("Game world cleaned up");
}

/// Handle basic input during gameplay
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Space pressed - implement action here");
    }
}

/// Simple camera movement with arrow keys during gameplay
fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        let movement_speed = 200.0;
        let movement_delta = movement_speed * time.delta_seconds();

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            camera_transform.translation.x -= movement_delta;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            camera_transform.translation.x += movement_delta;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            camera_transform.translation.y += movement_delta;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            camera_transform.translation.y -= movement_delta;
        }
    }
}
