mod components;
pub mod resources;
mod setup;
mod systems;

use self::{
    components::{OverlayButton, UiBackground, UiRootNode},
    setup::setup,
    systems::*,
};
use crate::{cleanup_system, states::AppState, systems::menu_button_hovered};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, SystemSet};
use interactable::InteractLabel;

const STATE: AppState = AppState::Overlay;

pub fn prepare_overlay(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(STATE).with_system(setup))
        .add_system_set(
            SystemSet::on_update(STATE)
                .with_system(button_system.after(InteractLabel::Interact))
                .with_system(menu_button_hovered::<OverlayButton>.after(InteractLabel::Interact))
                .with_system(hotkey_system)
                .with_system(window_resize_system),
        )
        .add_system_set(
            SystemSet::on_exit(STATE)
                .with_system(cleanup_system::<UiRootNode>)
                .with_system(cleanup_system::<UiBackground>),
        )
        // TODO: In theory, on_in_stack_update should be perfect but it doesn't seem to work
        .add_system_set(SystemSet::on_inactive_update(STATE).with_system(window_resize_system));
}
