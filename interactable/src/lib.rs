use bevy::prelude::{Component, Plugin};
use click::{
    click_system, MouseLeftJustEvent, MouseLeftPressedEvent, MouseLeftReleasedEvent,
    MouseRightJustEvent, MouseRightPressedEvent, MouseRightReleasedEvent,
};
use hover::{hover_system, MouseEnterEvent, MouseExitEvent, MouseOverEvent};

pub mod click;
mod common;
pub mod hover;
pub mod shapes;

#[derive(Component)]
pub struct InteractableCamera;

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<MouseOverEvent>()
            .add_event::<MouseEnterEvent>()
            .add_event::<MouseExitEvent>()
            .add_event::<MouseLeftJustEvent>()
            .add_event::<MouseLeftPressedEvent>()
            .add_event::<MouseLeftReleasedEvent>()
            .add_event::<MouseRightJustEvent>()
            .add_event::<MouseRightPressedEvent>()
            .add_event::<MouseRightReleasedEvent>()
            .add_system(hover_system)
            .add_system(click_system);
    }
}
