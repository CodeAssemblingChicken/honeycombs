mod common;
pub mod components;
pub mod shapes;
mod systems;

use bevy::prelude::{
    Commands, Component, Entity, ParallelSystemDescriptorCoercion, Plugin, SystemLabel,
};
use components::*;
use systems::interact_system;

#[derive(Component)]
pub struct InteractableCamera;

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(interact_system.label(InteractLabel::Interact));
    }
}

pub fn remove_interactable(commands: &mut Commands, e: Entity) {
    commands
        .entity(e)
        .remove::<Entered>()
        .remove::<Hovered>()
        .remove::<Exited>()
        .remove::<JustPressedLeft>()
        .remove::<PressedLeft>()
        .remove::<ReleasedLeft>()
        .remove::<JustPressedRight>()
        .remove::<PressedRight>()
        .remove::<ReleasedRight>()
        .remove::<JustPressedMiddle>()
        .remove::<PressedMiddle>()
        .remove::<ReleasedMiddle>()
        .remove::<Interactable>();
}

#[derive(SystemLabel)]
pub enum InteractLabel {
    Interact,
}
