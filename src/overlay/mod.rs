mod components;
pub mod resources;
mod setup;
mod systems;

use self::{setup::setup, systems::*};
use crate::states::AppState;
use bevy::prelude::{App, SystemSet};

const STATE: AppState = AppState::Overlay;

pub fn prepare_overlay(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(STATE).with_system(setup))
        .add_system_set(SystemSet::on_update(STATE).with_system(hotkey_system))
        .add_system_set(SystemSet::on_exit(STATE).with_system(cleanup));
}
