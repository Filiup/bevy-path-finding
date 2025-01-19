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
pub enum MenuState {
    #[default]
    WithoutMaze,
    WithMaze,
}

impl Default for MazeState {
    fn default() -> Self {
        Self::MainMenu(MenuState::default())
    }
}
