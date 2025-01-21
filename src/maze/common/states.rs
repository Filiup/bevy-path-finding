use bevy::prelude::States;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash)]

pub enum MazeState {
    MainMenu(MenuState),
    MazeGeneration,
    MazeSolving,
    MazeSave,
    MazeLoad,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
#[allow(clippy::enum_variant_names)]
pub enum MenuState {
    #[default]
    WithoutMaze,
    WithMaze,
    WithSolvedMaze,
}

impl Default for MazeState {
    fn default() -> Self {
        Self::MainMenu(MenuState::default())
    }
}
