mod components;
mod setup;
mod systems;

use self::{
    components::{UiBackground, UiRootNode},
    setup::setup,
    systems::*,
};
use crate::{cleanup_system, states::AppState};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, SystemSet};
use interactable::InteractLabel;

const STATE: AppState = AppState::Tutorial;

pub fn prepare_tutorial(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(STATE).with_system(setup))
        .add_system_set(
            SystemSet::on_update(STATE)
                .with_system(button_system.after(InteractLabel::Interact))
                .with_system(hotkey_system)
                .with_system(window_resize_system),
        )
        .add_system_set(
            SystemSet::on_exit(STATE)
                .with_system(cleanup_system::<UiRootNode>)
                .with_system(cleanup_system::<UiBackground>),
        );
}
