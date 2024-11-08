mod maze;

use bevy::app::App;
use maze::generation::MazeGenerationPlugin;
use maze::window::MazeWindowPlugin;

fn main() {
    App::new()
        .add_plugins(MazeWindowPlugin)
        .add_plugins(MazeGenerationPlugin)
        .run();
}
