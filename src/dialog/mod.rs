mod components;
pub mod resources;
mod setup;
mod systems;

use crate::{cleanup_system, states::AppState, systems::menu_button_hovered};
use bevy::prelude::{default, App, ParallelSystemDescriptorCoercion, SystemSet};
use interactable::InteractLabel;

use self::{
    components::UiRootNode,
    resources::DialogSettings,
    systems::{button_system, hotkey_system, window_resize_system},
};

const STATE: AppState = AppState::Dialog;

pub fn prepare_dialog(app: &mut App) {
    app.insert_resource(DialogSettings::default())
        .add_system_set(SystemSet::on_enter(STATE).with_system(setup::setup))
        .add_system_set(
            SystemSet::on_update(STATE)
                .with_system(button_system.after(InteractLabel::Interact))
                .with_system(menu_button_hovered.after(InteractLabel::Interact))
                .with_system(hotkey_system)
                .with_system(window_resize_system),
        )
        .add_system_set(SystemSet::on_exit(STATE).with_system(cleanup_system::<UiRootNode>));
}
