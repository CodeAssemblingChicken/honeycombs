use crate::shapes::{ContainsPoint, Shape};
use bevy::{
    math::Vec2,
    prelude::{Component, GlobalTransform},
};

#[derive(Default, Component)]
pub struct Interactable {
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
}

#[derive(Component)]
pub struct Hovered;
#[derive(Component)]
pub struct Entered;
#[derive(Component)]
pub struct Exited;

#[derive(Component)]
pub struct JustPressedLeft;
#[derive(Component)]
pub struct PressedLeft;
#[derive(Component)]
pub struct ReleasedLeft;

#[derive(Component)]
pub struct JustPressedRight;
#[derive(Component)]
pub struct PressedRight;
#[derive(Component)]
pub struct ReleasedRight;

#[derive(Component)]
pub struct JustPressedMiddle;
#[derive(Component)]
pub struct PressedMiddle;
#[derive(Component)]
pub struct ReleasedMiddle;
