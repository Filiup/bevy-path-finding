mod maze;

use bevy::app::App;
use bevy::prelude::AppExtStates;
use maze::common::generation::MazeGenerationTimer;
use maze::common::states::MazeState;
use maze::default::MazeDefaultPlugin;
use maze::generation::MazeGenerationPlugin;
use maze::grid::MazeGridPlugin;
use maze::solving::MazeSolvingPlugin;
use maze::storage::MazeStoragePlugin;
use maze::ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(MazeDefaultPlugin)
        .init_state::<MazeState>()
        .init_resource::<MazeGenerationTimer>()
        .add_plugins(MazeStoragePlugin)
        .add_plugins(MazeGridPlugin)
        .add_plugins(MazeGenerationPlugin)
        .add_plugins(MazeSolvingPlugin)
        .add_plugins(UiPlugin)
        .run();
}
