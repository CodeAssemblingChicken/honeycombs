// pub mod click;
mod common;
// pub mod hover;
pub mod shapes;
pub mod structs;
mod systems;

use bevy::prelude::{Component, ParallelSystemDescriptorCoercion, Plugin, SystemLabel};
// use click::{click_system, MouseLeftClickEvent, MouseMiddleClickEvent, MouseRightClickEvent};
// use hover::{hover_system, MouseEnterEvent, MouseExitEvent, MouseOverEvent};
use systems::interact_system;

#[derive(Component)]
pub struct InteractableCamera;

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.add_event::<MouseOverEvent>()
        //     .add_event::<MouseEnterEvent>()
        //     .add_event::<MouseExitEvent>()
        //     .add_event::<MouseLeftClickEvent>()
        //     .add_event::<MouseRightClickEvent>()
        //     .add_event::<MouseMiddleClickEvent>()
        //     .add_system(hover_system)
        //     .add_system(click_system);
        app.add_system(interact_system.label(InteractLabel::Interact));
    }
}

#[derive(SystemLabel)]
pub enum InteractLabel {
    Interact,
}
