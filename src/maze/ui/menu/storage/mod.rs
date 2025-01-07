use crate::maze::constants::ui::{
    NOT_EMPTY_SAVE_SLOT_BUTTON_COLOR, SAVE_SLOT_BUTTON_COLOR, SAVE_SLOT_BUTTON_HEIGHT,
    SAVE_SLOT_BUTTON_WIDTH,
};

pub(crate) use super::*;
pub mod load;
pub mod save;

use load::LoadMenuPlugin;
use save::SaveMenuPlugin;

#[derive(Component)]
pub struct SaveSlotButton;

#[derive(Component)]
pub struct LoadSlotButton;

#[derive(Component, Clone, Copy)]
pub struct StorageSlot {
    pub slot: usize,
    pub empty: bool,
}

impl StorageSlot {
    pub fn new(slot: usize, empty: bool) -> StorageSlot {
        StorageSlot { slot, empty }
    }
}

fn spawn_storage_slot_button<'a>(
    builder: &'a mut ChildBuilder,
    save_slot: StorageSlot,
    button_kind_componmment: impl Component,
) -> EntityCommands<'a> {
    let color = if save_slot.empty {
        SAVE_SLOT_BUTTON_COLOR
    } else {
        NOT_EMPTY_SAVE_SLOT_BUTTON_COLOR
    };

    spawn_button(
        builder,
        button_kind_componmment,
        save_slot,
        SAVE_SLOT_BUTTON_WIDTH,
        SAVE_SLOT_BUTTON_HEIGHT,
        color,
        &save_slot.slot.to_string(),
    )
}

pub struct StorageMenuPlugin;
impl Plugin for StorageMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SaveMenuPlugin).add_plugins(LoadMenuPlugin);
    }
}
