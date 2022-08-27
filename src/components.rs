use crate::constants::{SCALE_ENLARGED, SCALE_NORMAL};
use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Entity, Handle, Query, Transform},
    sprite::ColorMaterial,
};
use bevy_easings::{Ease, EaseFunction, EasingType};
use std::time::Duration;

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

// TODO: Maybe use systems and events instead?
// e.g. CellHoverEvent(entity)
impl Cell {
    pub fn hover(
        &mut self,
        commands: &mut Commands,
        light: Handle<ColorMaterial>,
        dark: Handle<ColorMaterial>,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
    ) {
        if self.hovering {
            return;
        }
        self.hovering = true;
        // Enlarge
        self.rescale(commands, SCALE_ENLARGED);
        // Set colors to hovering
        self.set_colors(light, dark, color_query);
    }

    pub fn unhover(
        &mut self,
        commands: &mut Commands,
        light: Handle<ColorMaterial>,
        dark: Handle<ColorMaterial>,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
    ) {
        if !self.hovering {
            return;
        }
        self.hovering = false;
        // Normal scale
        self.rescale(commands, SCALE_NORMAL);
        // Set colors to normal
        self.set_colors(light, dark, color_query);
    }

    pub fn click(
        &mut self,
        commands: &mut Commands,
        light: Handle<ColorMaterial>,
        dark: Handle<ColorMaterial>,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
    ) {
        self.rescale(commands, SCALE_NORMAL);
        self.set_colors(light, dark, color_query);
    }

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
        light: Handle<ColorMaterial>,
        dark: Handle<ColorMaterial>,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
    ) {
        // Get Material Handles from the children
        color_query
            .get_mut(self.outer_hexagon)
            .and_then(|mut h| Ok(*h = dark))
            .unwrap();
        color_query
            .get_mut(self.inner_hexagon)
            .and_then(|mut h| Ok(*h = light))
            .unwrap();
        // unwrap should be fine, because if the children exist they're also in the query
    }
}

/// Used for querying only the inner hexes
#[derive(Debug, Component)]
pub struct CellInner;
/// Used for querying only the outer hexes
#[derive(Debug, Component)]
pub struct CellOuter;
