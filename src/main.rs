mod maze;

use bevy::app::App;
use bevy::prelude::AppExtStates;
use maze::generation::MazeGenerationPlugin;
use maze::grid::MazeGridPlugin;
use maze::states::MazeState;
use maze::ui::UiPlugin;
use maze::window::MazeWindowPlugin;

fn main() {
    App::new()
        .add_plugins(MazeWindowPlugin)
        .init_state::<MazeState>()
        .add_plugins(MazeGridPlugin)
        .add_plugins(MazeGenerationPlugin)
        .add_plugins(UiPlugin)
        .run();
}
