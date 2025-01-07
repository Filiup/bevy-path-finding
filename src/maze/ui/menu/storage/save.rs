use super::*;
use bevy::prelude::*;
use std::path::Path;

use super::{despawn_menu, spawn_slot_container, spawn_ui_container};
use crate::maze::{
    common::states::{MazeState, MenuState},
    storage::SaveMazeEvent,
};

#[derive(Component)]
pub struct SaveMenu;

#[allow(clippy::type_complexity)]
pub fn save_maze(
    storage_slot_interraction_query: Query<
        (&Interaction, &StorageSlot),
        (Changed<Interaction>, With<SaveSlotButton>),
    >,
    mut maze_next_state: ResMut<NextState<MazeState>>,
    mut save_maze_writer: EventWriter<SaveMazeEvent>,
) {
    let pressed_slot = storage_slot_interraction_query
        .iter()
        .find(|(&interraction, _)| interraction == Interaction::Pressed);

    if let Some((_, storage_slot)) = pressed_slot {
        save_maze_writer.send(SaveMazeEvent {
            slot: storage_slot.slot,
        });
        maze_next_state.set(MazeState::MainMenu(MenuState::WithMaze));
    }
}

pub fn build_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands, SaveMenu)
        .with_children(|builder| {
            builder.spawn((
                TextLayout::new_with_justify(JustifyText::Center),
                Text::new("Select save location"),
            ));
        })
        .with_children(|builder| {
            spawn_slot_container(builder).with_children(|builder| {
                for order in 1..11 {
                    let save_path = format!("saves/save_{}.mz", order);
                    let save_slot = if Path::new(&save_path).exists() {
                        StorageSlot::new(order, false)
                    } else {
                        StorageSlot::new(order, true)
                    };

                    spawn_storage_slot_button(builder, save_slot, SaveSlotButton);
                }
            });
        });
}

pub(crate) struct SaveMenuPlugin;

impl Plugin for SaveMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MazeState::MazeSave), build_menu)
            .add_systems(OnExit(MazeState::MazeSave), despawn_menu::<SaveMenu>)
            .add_systems(Update, save_maze);
    }
}
