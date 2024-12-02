pub mod builders;
pub mod interactions;

use bevy::prelude::*;
use interactions::buttons_hover_changle_color;

#[derive(Component)]
pub struct ActionButton;

pub struct ActionButtonsPlugin;

impl Plugin for ActionButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, buttons_hover_changle_color);
    }
}
