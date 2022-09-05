mod board;
mod components;
mod setup;
mod systems;

use self::{setup::setup, systems::*};
use crate::{cleanup, states::AppState};
use bevy::{
    app::App,
    prelude::{ParallelSystemDescriptorCoercion, SystemSet},
};

const STATE: AppState = AppState::Level;

pub fn prepare_level(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(STATE).with_system(setup))
        .add_system_set(
            SystemSet::on_update(STATE)
                .with_system(mouse_enter_cell)
                .with_system(mouse_exit_cell.before(mouse_enter_cell))
                .with_system(
                    mouse_click_cell
                        .after(mouse_enter_cell)
                        .after(mouse_exit_cell),
                )
                .with_system(check_solved)
                .with_system(window_resize_system)
                .with_system(hotkey_system),
        )
        // TODO: In theory, on_in_stack_update should be perfect but it doesn't seem to work
        .add_system_set(SystemSet::on_inactive_update(STATE).with_system(window_resize_system))
        .add_system_set(SystemSet::on_pause(STATE).with_system(pause))
        .add_system_set(SystemSet::on_exit(STATE).with_system(cleanup));
}
