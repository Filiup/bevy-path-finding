mod maze;

use bevy::app::App;
use maze::ui::UiPlugin;
use maze::generation::MazeGenerationPlugin;
use maze::window::MazeWindowPlugin;

fn main() {
    App::new()
        .add_plugins(MazeWindowPlugin)
        .add_plugins(MazeGenerationPlugin)
        .add_plugins(UiPlugin)
        .run();
}
