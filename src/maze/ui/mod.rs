mod main_menu;
mod menu;

mod buttons;
mod sliders;

use bevy::prelude::*;

use buttons::ActionButtonsPlugin;
use menu::MenuPlugin;
use sliders::SlidersPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ActionButtonsPlugin)
            .add_plugins(SlidersPlugin)
            .add_plugins(MenuPlugin);
    }
}
