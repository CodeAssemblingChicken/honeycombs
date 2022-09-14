mod components;
mod constants;
mod setup;
mod systems;

use self::{setup::setup, systems::*};
use crate::{cleanup_system, components::RootComponent, states::AppState};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, SystemSet};
use interactable::InteractLabel;

const STATE: AppState = AppState::Options;

pub fn prepare_settings(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(STATE).with_system(setup))
        .add_system_set(
            SystemSet::on_update(STATE)
                .with_system(lang_click_system.after(InteractLabel::Interact))
                .with_system(mouse_setting_click_system.after(InteractLabel::Interact))
                .with_system(lang_hover_system.after(InteractLabel::Interact))
                .with_system(mouse_setting_hover_system.after(InteractLabel::Interact))
                .with_system(window_resize_system),
        )
        .add_system_set(SystemSet::on_exit(STATE).with_system(cleanup_system::<RootComponent>));
}
