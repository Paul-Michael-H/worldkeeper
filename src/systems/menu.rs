use bevy::prelude::*;
use crate::components::ui::*;
use crate::resources::GameState;

/// Setup the main menu UI
pub fn setup_main_menu(mut commands: Commands) {
    info!("Setting up main menu");
    
    // Spawn a camera for UI rendering
    commands.spawn(Camera2dBundle::default());
    
    // Main menu root container
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: MenuTheme::BACKGROUND_COLOR.into(),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                TextBundle::from_section(
                    "WorldKeeper",
                    TextStyle {
                        font_size: MenuTheme::TITLE_FONT_SIZE,
                        color: MenuTheme::TITLE_COLOR,
                        ..default()
                    },
                ),
                MenuTitle,
            ));
            
            // Subtitle
            parent.spawn(TextBundle::from_section(
                "God Simulation Game",
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ));
            
            // Button container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(15.0),
                        margin: UiRect::top(Val::Px(40.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // New Worldkeeper button
                    create_menu_button(
                        parent,
                        "New Worldkeeper",
                        MenuAction::NewWorldkeeper,
                    );
                    
                    // Start New Game button
                    create_menu_button(
                        parent,
                        "Start New Game",
                        MenuAction::StartNewGame,
                    );
                });
        });
}

/// Helper function to create a menu button
fn create_menu_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: MenuAction,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: MenuTheme::BUTTON_NORMAL.into(),
                ..default()
            },
            MenuButton::new(action),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: MenuTheme::BUTTON_FONT_SIZE,
                        color: MenuTheme::BUTTON_TEXT,
                        ..default()
                    },
                ),
                ButtonText,
            ));
        });
}

/// Handle button interactions (hover effects)
pub fn handle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MenuButton>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = MenuTheme::BUTTON_PRESSED.into();
            }
            Interaction::Hovered => {
                *background_color = MenuTheme::BUTTON_HOVERED.into();
            }
            Interaction::None => {
                *background_color = MenuTheme::BUTTON_NORMAL.into();
            }
        }
    }
}

/// Handle button clicks and trigger state changes
pub fn handle_menu_actions(
    mut interaction_query: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button.action {
                MenuAction::NewWorldkeeper => {
                    info!("New Worldkeeper selected");
                    next_state.set(GameState::NewWorldkeeper);
                }
                MenuAction::StartNewGame => {
                    info!("Start New Game selected");
                    next_state.set(GameState::InGame);
                }
                MenuAction::Settings => {
                    info!("Settings selected");
                    // TODO: Implement settings menu
                }
                MenuAction::Quit => {
                    info!("Quit selected");
                    exit.send(AppExit::Success);
                }
            }
        }
    }
}

/// Cleanup main menu when transitioning to other states
pub fn cleanup_main_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenu>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    for entity in &menu_query {
        commands.entity(entity).despawn_recursive();
    }
    
    // Also cleanup the UI camera
    for entity in &camera_query {
        commands.entity(entity).despawn();
    }
    
    info!("Main menu cleaned up");
}

/// Handle escape key to return to main menu from other states
pub fn handle_escape_to_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::InGame | GameState::NewWorldkeeper => {
                info!("Returning to main menu");
                next_state.set(GameState::MainMenu);
            }
            _ => {}
        }
    }
}