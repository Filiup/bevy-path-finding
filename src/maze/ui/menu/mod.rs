use bevy::prelude::Plugin;
use main::MainMenuPlugin;
pub mod main;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(MainMenuPlugin);
    }
}
