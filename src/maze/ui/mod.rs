mod menu;

mod buttons;
mod sliders;

use bevy::prelude::*;

use menu::MenuPlugin;
use sliders::SlidersPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SlidersPlugin).add_plugins(MenuPlugin);
    }
}
