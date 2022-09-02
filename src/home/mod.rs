mod components;
mod functions;
mod setup;
mod systems;

use self::{setup::setup, systems::*};
use crate::{cleanup, states::AppState};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, SystemSet};

const STATE: AppState = AppState::Home;

pub fn prepare_home(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(STATE).with_system(setup))
        .add_system_set(
            SystemSet::on_update(STATE)
                .with_system(mouse_enter_cell)
                .with_system(mouse_exit_cell.before(mouse_enter_cell))
                .with_system(mouse_click_cell.after(mouse_enter_cell))
                .with_system(window_resize_system),
        )
        .add_system_set(SystemSet::on_exit(STATE).with_system(cleanup));
}
