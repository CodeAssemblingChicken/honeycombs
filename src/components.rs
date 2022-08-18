use crate::interactable::hover::Hoverable;
use bevy::{
    asset::HandleId,
    math::Vec3,
    prelude::{
        Bundle, ColorMaterial, Commands, Component, ComputedVisibility, Entity, GlobalTransform,
        Handle, Query, ResMut, Transform, Visibility,
    },
    sprite::Mesh2dHandle,
};
use bevy_easings::{Ease, EaseFunction, EasingType};
use std::time::Duration;

const SCALE_NORMAL: Vec3 = Vec3::new(1., 1., 1.);
const SCALE_ENLARGED: Vec3 = Vec3::new(1.06, 1.06, 1.);

#[derive(Component)]
pub struct MainCamera;

#[derive(Bundle)]
pub struct CellBundle {
    pub cell: Cell,

    pub mesh: Mesh2dHandle,
    pub material: Handle<ColorMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl Default for CellBundle {
    fn default() -> Self {
        Self {
            cell: Default::default(),
            mesh: Default::default(),
            material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}

// TODO: Make Hexagon Entities not Option
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Clone, Default)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub entity: Option<Entity>,
    pub outer_hexagon: Option<Entity>,
    pub inner_hexagon: Option<Entity>,
}

impl Cell {
    pub fn hover(
        &self,
        commands: &mut Commands,
        t: &Transform,
        child_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &ResMut<CellColors>,
    ) {
        // Enlarge
        self.rescale(commands, t, SCALE_ENLARGED);
        // Set colors to hovering
        self.set_colors(
            cell_colors.yellow_medium.id,
            cell_colors.yellow_dark.id,
            child_query,
        );
    }

    pub fn unhover(
        &self,
        commands: &mut Commands,
        t: &Transform,
        child_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &ResMut<CellColors>,
    ) {
        // Enlarge
        self.rescale(commands, t, SCALE_NORMAL);
        // Set colors to hovering
        self.set_colors(
            cell_colors.yellow_light.id,
            cell_colors.yellow_medium.id,
            child_query,
        );
    }

    fn rescale(&self, commands: &mut Commands, orig: &Transform, scale: Vec3) {
        // Rescale hexagon to desired scale by easing
        let mut t1 = orig.clone();
        t1.scale = scale;
        commands.entity(self.entity.unwrap()).insert(orig.ease_to(
            t1,
            EaseFunction::ElasticOut,
            EasingType::Once {
                duration: Duration::from_millis(300),
            },
        ));
    }

    fn set_colors(
        &self,
        light: HandleId,
        dark: HandleId,
        child_query: &mut Query<&mut Handle<ColorMaterial>>,
    ) {
        // TODO: Make hexagons not Option so no unwrap is needed
        // Get Material Handles from the children
        child_query
            .get_mut(self.outer_hexagon.unwrap())
            .and_then(|mut h| Ok(h.id = dark))
            .unwrap();
        child_query
            .get_mut(self.inner_hexagon.unwrap())
            .and_then(|mut h| Ok(h.id = light))
            .unwrap();
        // The final unwraps should be fine, because if the children exist they're also in the query
    }
}

#[derive(Bundle)]
pub struct HiddenCell {
    pub hoverable: Hoverable,
}

#[derive(Debug, Component)]
pub struct NumberCell {
    pub count: u8,
}

#[derive(Debug, Component)]
pub struct EmptyCell;

#[derive(Debug, Component)]
pub struct CellInner;
#[derive(Debug, Component)]
pub struct CellOuter;

pub struct CellColors {
    pub white: Handle<ColorMaterial>,
    pub yellow_dark: Handle<ColorMaterial>,
    pub yellow_medium: Handle<ColorMaterial>,
    pub yellow_light: Handle<ColorMaterial>,
    pub gray_dark: Handle<ColorMaterial>,
    pub gray_light: Handle<ColorMaterial>,
    pub blue_dark: Handle<ColorMaterial>,
    pub blue_light: Handle<ColorMaterial>,
}
