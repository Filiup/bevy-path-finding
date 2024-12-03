pub(crate) mod with_maze;
pub(crate) mod without_maze;

use with_maze::WithMazeMenuPlugin;
use without_maze::WithoutMazeMenuPlugin;

pub(crate) use super::*;
pub(crate) use crate::maze::ui::buttons::builders::spawn_action_button;
pub(crate) use crate::maze::ui::buttons::interactions::button_state_system;

#[derive(Component)]
pub(crate) struct GenerateMazeButton;

#[derive(Component)]
pub(crate) struct LoadMazeButton;

#[derive(Component)]
pub(crate) struct SaveMazeButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WithoutMazeMenuPlugin)
            .add_plugins(WithMazeMenuPlugin);
    }
}
