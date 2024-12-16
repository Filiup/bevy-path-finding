use std::path::Path;

use bevy::prelude::*;

use super::{despawn_menu, main::spawn_button, spawn_ui_container};
use crate::maze::{
    common::states::{MazeState, MenuState},
    constants::ui::{
        NOT_EMPTY_SAVE_SLOT_BUTTON_COLOR, SAVE_SLOT_BUTTON_COLOR, SAVE_SLOT_BUTTON_HEIGHT,
        SAVE_SLOT_BUTTON_WIDTH,
    },
    storage::SaveMazeEvent,
};

#[derive(Component)]
pub struct SaveMenu;

#[derive(Component)]
pub struct SaveSlotButton;

#[derive(Component, Clone, Copy)]
pub struct SaveSlot {
    pub slot: usize,
    pub empty: bool,
}

impl SaveSlot {
    pub fn new(slot: usize, empty: bool) -> SaveSlot {
        SaveSlot { slot, empty }
    }
}

pub fn save_maze(
    maze_slot_interraction_query: Query<(&Interaction, &SaveSlot), Changed<Interaction>>,
    mut maze_next_state: ResMut<NextState<MazeState>>,
    mut save_maze_writer: EventWriter<SaveMazeEvent>,
) {
    let pressed_slot = maze_slot_interraction_query
        .iter()
        .find(|(&interraction, _)| interraction == Interaction::Pressed);

    if let Some((_, save_slot)) = pressed_slot {
        save_maze_writer.send(SaveMazeEvent {
            slot: save_slot.slot,
        });
        maze_next_state.set(MazeState::MainMenu(MenuState::WithMaze));
    }
}

fn spawn_save_container<'a>(builder: &'a mut ChildBuilder) -> EntityCommands<'a> {
    let container = builder.spawn(Node {
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
        grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
        row_gap: Val::Px(10.0),
        column_gap: Val::Px(10.0),
        ..default()
    });

    container
}

fn spawn_save_slot_button<'a>(
    builder: &'a mut ChildBuilder,
    save_slot: SaveSlot,
) -> EntityCommands<'a> {
    let color = if save_slot.empty {
        SAVE_SLOT_BUTTON_COLOR
    } else {
        NOT_EMPTY_SAVE_SLOT_BUTTON_COLOR
    };

    spawn_button(
        builder,
        SaveSlotButton,
        save_slot,
        SAVE_SLOT_BUTTON_WIDTH,
        SAVE_SLOT_BUTTON_HEIGHT,
        color,
        &save_slot.slot.to_string(),
    )
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
            spawn_save_container(builder).with_children(|builder| {
                for order in 1..11 {
                    let save_path = format!("saves/save_{}.mz", order);
                    let save_slot = if Path::new(&save_path).exists() {
                        SaveSlot::new(order, false)
                    } else {
                        SaveSlot::new(order, true)
                    };

                    spawn_save_slot_button(builder, save_slot);
                }
            });
        });
}

pub struct SaveMenuPlugin;

impl Plugin for SaveMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MazeState::MazeSave), build_menu)
            .add_systems(OnExit(MazeState::MazeSave), despawn_menu::<SaveMenu>)
            .add_systems(Update, save_maze);
    }
}
