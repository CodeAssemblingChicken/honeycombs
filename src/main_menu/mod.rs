pub mod menu;
pub mod systems;

use self::{menu::setup, systems::*};
use crate::states::AppState;
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, SystemSet};

const STATE: AppState = AppState::MainMenu;

pub fn prepare_main_menu(app: &mut App) {
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
                .with_system(window_resize_system),
        )
        .add_system_set(SystemSet::on_exit(STATE));
}
