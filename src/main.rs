use bevy::prelude::*;

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
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            move_camera,
        ))
        .run();
}

/// Initial setup system
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn a simple colored rectangle as a placeholder
    commands.spawn(ColorMesh2dBundle {
        mesh: meshes.add(Rectangle::new(100.0, 100.0)).into(),
        material: materials.add(Color::srgb(0.8, 0.2, 0.3)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });

    info!("WorldKeeper initialized!");
}

/// Handle basic input
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        info!("Escape pressed - implement exit or menu here");
    }
    
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Space pressed - implement action here");
    }
}

/// Simple camera movement with arrow keys
fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.single_mut();
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
