use bevy::prelude::*;
use crate::resources::GameState;
use crate::systems::menu::*;

/// Plugin for managing the main menu and UI interactions
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add state management
            .init_state::<GameState>()
            
            // Systems that run when entering MainMenu state
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            
            // Systems that run while in MainMenu state
            .add_systems(
                Update,
                (
                    handle_button_interactions,
                    handle_menu_actions,
                    handle_escape_to_menu,
                ).run_if(in_state(GameState::MainMenu))
            )
            
            // Systems that run when exiting MainMenu state
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);
    }
}

/// Plugin for handling the "New Worldkeeper" screen
pub struct NewWorldkeeperPlugin;

impl Plugin for NewWorldkeeperPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::NewWorldkeeper), setup_new_worldkeeper_screen)
            .add_systems(
                Update,
                handle_escape_to_menu.run_if(in_state(GameState::NewWorldkeeper))
            )
            .add_systems(OnExit(GameState::NewWorldkeeper), cleanup_new_worldkeeper_screen);
    }
}

/// Temporary setup for the New Worldkeeper screen
fn setup_new_worldkeeper_screen(mut commands: Commands) {
    info!("Setting up New Worldkeeper screen");
    
    // Spawn a camera for UI rendering
    commands.spawn(Camera2dBundle::default());
    
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgb(0.1, 0.2, 0.1).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "New Worldkeeper Screen",
                TextStyle {
                    font_size: 48.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            
            parent.spawn(TextBundle::from_section(
                "Press ESC to return to main menu",
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ));
        });
}

/// Cleanup New Worldkeeper screen
fn cleanup_new_worldkeeper_screen(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Node>, With<Camera>)>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    info!("New Worldkeeper screen cleaned up");
}