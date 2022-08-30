mod setup;

use self::setup::setup;
use crate::{cleanup, states::AppState};
use bevy::prelude::{App, SystemSet};

const STATE: AppState = AppState::EndScreen;

pub fn prepare_end_screen(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(STATE).with_system(setup))
        .add_system_set(SystemSet::on_update(STATE))
        .add_system_set(SystemSet::on_exit(STATE).with_system(cleanup));
}
