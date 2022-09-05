mod board;
mod components;
mod functions;
mod setup;
mod systems;

use self::{components::CellUpdateEvent, setup::setup, systems::*};
use crate::{cleanup, states::AppState};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, SystemSet};

const STATE: AppState = AppState::Editor;

pub fn prepare_editor(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(STATE).with_system(setup))
        .add_system_set(
            SystemSet::on_update(STATE)
                .with_system(mouse_enter_cell)
                .with_system(mouse_exit_cell)
                .with_system(
                    mouse_click_unset_cell
                        .after(mouse_enter_cell)
                        .after(mouse_exit_cell),
                )
                .with_system(
                    mouse_click_empty_cell
                        .before(mouse_click_unset_cell)
                        .after(mouse_enter_cell)
                        .after(mouse_exit_cell),
                )
                .with_system(
                    mouse_click_number_cell
                        .before(mouse_click_unset_cell)
                        .after(mouse_enter_cell)
                        .after(mouse_exit_cell),
                )
                .with_system(
                    cell_update_system
                        .before(mouse_click_empty_cell)
                        .before(mouse_click_number_cell),
                )
                .with_system(hotkey_system)
                .with_system(window_resize_system),
        )
        .add_system_set(SystemSet::on_exit(STATE).with_system(cleanup))
        .add_event::<CellUpdateEvent>();
}
