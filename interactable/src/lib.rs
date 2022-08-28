pub mod click;
mod common;
pub mod hover;
pub mod shapes;

use bevy::prelude::{Component, Plugin};
use click::{click_system, MouseLeftClickEvent, MouseMiddleClickEvent, MouseRightClickEvent};
use hover::{hover_system, MouseEnterEvent, MouseExitEvent, MouseOverEvent};

#[derive(Component)]
pub struct InteractableCamera;

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<MouseOverEvent>()
            .add_event::<MouseEnterEvent>()
            .add_event::<MouseExitEvent>()
            .add_event::<MouseLeftClickEvent>()
            .add_event::<MouseRightClickEvent>()
            .add_event::<MouseMiddleClickEvent>()
            .add_system(hover_system)
            .add_system(click_system);
    }
}
