use crate::enums::{HintDirection, HintType};
use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Entity, Handle, Query, Transform},
    sprite::ColorMaterial,
};
use bevy_easings::{Ease, EaseFunction, EasingType};
use serde::{Deserialize, Serialize};
use std::time::Duration;

// TODO: This is probably way to big
/// Cell component storing everything cell related
#[cfg_attr(
    feature = "bevy-inspector-egui",
    derive(bevy_inspector_egui::Inspectable)
)]
#[derive(Component, Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub entity: Entity,
    pub outer_hexagon: Entity,
    pub inner_hexagon: Entity,
    pub orig: Transform,
}

impl Cell {
    pub fn hover(
        &mut self,
        commands: &mut Commands,
        background: Option<Handle<ColorMaterial>>,
        light: Handle<ColorMaterial>,
        dark: Handle<ColorMaterial>,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
    ) {
        // Enlarge
        self.rescale(commands, self.orig.scale * Vec3::new(1.04, 1.04, 1.));
        // Set colors to hovering
        self.set_colors(background, light, dark, color_query);
    }

    pub fn unhover(
        &mut self,
        commands: &mut Commands,
        background: Option<Handle<ColorMaterial>>,
        light: Handle<ColorMaterial>,
        dark: Handle<ColorMaterial>,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
    ) {
        // Normal scale
        self.rescale(commands, self.orig.scale);
        // Set colors to normal
        self.set_colors(background, light, dark, color_query);
    }

    pub fn click(
        &mut self,
        commands: &mut Commands,
        background: Option<Handle<ColorMaterial>>,
        light: Handle<ColorMaterial>,
        dark: Handle<ColorMaterial>,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
    ) {
        self.rescale(commands, self.orig.scale);
        self.set_colors(background, light, dark, color_query);
    }

    /// Common function for easing the scale to a given value
    pub fn rescale(&self, commands: &mut Commands, scale: Vec3) {
        // Rescale hexagon to desired scale by easing
        let mut t1 = self.orig;
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
        background: Option<Handle<ColorMaterial>>,
        light: Handle<ColorMaterial>,
        dark: Handle<ColorMaterial>,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
    ) {
        // Get Material Handles from the children
        color_query
            .get_mut(self.outer_hexagon)
            .map(|mut h| *h = dark)
            .unwrap();
        color_query
            .get_mut(self.inner_hexagon)
            .map(|mut h| *h = light)
            .unwrap();
        if let Some(b) = background {
            color_query
                .get_mut(self.entity)
                .map(|mut h| *h = b)
                .unwrap();
        }
        // unwrap should be fine, because if the children exist they're also in the query
    }
}

/// Used for querying only the inner hexes
#[derive(Debug, Component)]
pub struct CellInner;
/// Used for querying only the outer hexes
#[derive(Debug, Component)]
pub struct CellOuter;

/// Component for column hints
#[derive(Debug, Component, Clone, Copy)]
pub struct ColumnHint {
    pub x: usize,
    pub y: usize,
    pub dir: HintDirection,
    pub hint_type: HintType,
}

#[derive(Component)]
pub struct RootComponent;

#[derive(Component)]
pub struct MenuButton;

#[derive(Default, Debug, Clone, Copy, Component, Serialize, Deserialize, PartialEq, Eq)]
pub enum Language {
    #[default]
    EN,
    DE,
    FR,
    ES,
}
