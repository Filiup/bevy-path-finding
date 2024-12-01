pub mod builders;
pub mod interactions;

use bevy::prelude::*;
use interactions::buttons_hover_changle_color;

pub const BUTTON_COLOR: Color = Color::srgb(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0);
pub const BUTTON_HOVER_COLOR: Color = Color::srgb(173.0 / 255.0, 216.0 / 255.0, 230.0 / 255.0);

#[derive(Component)]
pub struct ActionButton;

pub struct ActionButtonsPlugin;

impl Plugin for ActionButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, buttons_hover_changle_color);
    }
}
