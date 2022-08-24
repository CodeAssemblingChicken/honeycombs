use std::time::Duration;

use bevy::{
    asset::HandleId,
    math::Vec3,
    prelude::{Commands, Component, Entity, Handle, Query, Transform},
    sprite::ColorMaterial,
};
use bevy_easings::{Ease, EaseFunction, EasingType};

// TODO: This is probably way to big
/// Cell component storing everythin cell related
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Clone)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub entity: Entity,
    pub outer_hexagon: Entity,
    pub inner_hexagon: Entity,
    pub orig: Transform,
    pub hovering: bool,
}
impl Cell {
    /// Common function for easing the scale to a given value
    pub fn rescale(&self, commands: &mut Commands, scale: Vec3) {
        // Rescale hexagon to desired scale by easing
        let mut t1 = self.orig.clone();
        t1.scale = scale;
        commands.entity(self.entity).insert(self.orig.ease_to(
            t1,
            EaseFunction::ElasticOut,
            EasingType::Once {
                duration: Duration::from_millis(300),
            },
        ));
    }

    /// Common function for setting the color of the inner hexes
    pub fn set_colors(
        &self,
        light: HandleId,
        dark: HandleId,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
    ) {
        // Get Material Handles from the children
        color_query
            .get_mut(self.outer_hexagon)
            .and_then(|mut h| Ok(h.id = dark))
            .unwrap();
        color_query
            .get_mut(self.inner_hexagon)
            .and_then(|mut h| Ok(h.id = light))
            .unwrap();
        // unwrap should be fine, because if the children exist they're also in the query
    }
}
