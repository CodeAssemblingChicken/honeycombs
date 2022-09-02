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
                .with_system(mouse_over_cell)
                .with_system(mouse_enter_cell.before(mouse_over_cell))
                .with_system(mouse_exit_cell.before(mouse_enter_cell))
                .with_system(
                    mouse_click_cell
                        .after(mouse_enter_cell)
                        .after(mouse_exit_cell),
                )
                .with_system(window_resize_system)
                .with_system(check_solved)
                .with_system(hotkey_system),
        )
        .add_system_set(SystemSet::on_exit(STATE).with_system(cleanup));
}
