use bevy::{
    math::Vec2,
    prelude::{Component, GlobalTransform},
};

use crate::shapes::{ContainsPoint, Shape};

#[derive(Default, Component)]
pub struct Interactable {
    pub clicks: ClickAction,
    pub hovers: HoverAction,
    pub ignore_scale: bool,
    pub pass_through: bool,
    pub shape: Shape,
}

impl Interactable {
    pub fn contains_point(&self, point: Vec2, tf: &GlobalTransform) -> bool {
        let scaling = match self.ignore_scale {
            true => None,
            false => Some(tf.affine().to_scale_rotation_translation().0.truncate()),
        };
        self.shape
            .contains_point(point, tf.translation().truncate(), scaling)
    }
    pub fn reset(&mut self) {
        self.clicks = ClickAction::default();
        self.hovers = HoverAction::default();
    }
}

#[derive(Default)]
pub struct ClickAction {
    pub left_just: bool,
    pub left_pressed: bool,
    pub left_released: bool,
    pub right_just: bool,
    pub right_pressed: bool,
    pub right_released: bool,
    pub middle_just: bool,
    pub middle_pressed: bool,
    pub middle_released: bool,
}

#[derive(Default)]
pub struct HoverAction {
    pub entered: bool,
    pub hovered: bool,
    pub exited: bool,
}
