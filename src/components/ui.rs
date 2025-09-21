use bevy::prelude::*;

/// Marker component for the main menu UI
#[derive(Component)]
pub struct MainMenu;

/// Marker component for the main menu title
#[derive(Component)]
pub struct MenuTitle;

/// Component for menu buttons with their associated actions
#[derive(Component, Debug, Clone, PartialEq)]
pub struct MenuButton {
    pub action: MenuAction,
}

/// Actions that can be triggered by menu buttons
#[derive(Debug, Clone, PartialEq)]
pub enum MenuAction {
    NewWorldkeeper,
    StartNewGame,
    Settings,
    Quit,
}

impl MenuButton {
    pub fn new(action: MenuAction) -> Self {
        Self { action }
    }
}

/// Component for button text
#[derive(Component)]
pub struct ButtonText;

/// UI style constants for consistent theming
pub struct MenuTheme;

impl MenuTheme {
    pub const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
    pub const TITLE_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
    pub const BUTTON_NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const BUTTON_HOVERED: Color = Color::srgb(0.25, 0.25, 0.25);
    pub const BUTTON_PRESSED: Color = Color::srgb(0.35, 0.25, 0.35);
    pub const BUTTON_TEXT: Color = Color::srgb(0.9, 0.9, 0.9);
    
    pub const TITLE_FONT_SIZE: f32 = 64.0;
    pub const BUTTON_FONT_SIZE: f32 = 32.0;
}