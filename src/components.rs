use crate::interactable::{click::Clickable, hover::Hoverable};
use bevy::{
    asset::HandleId,
    audio::{AudioSink, AudioSource},
    math::Vec3,
    prelude::{
        Bundle, ColorMaterial, Commands, Component, Entity, Handle, Query, ResMut, Transform,
    },
};
use bevy_easings::{Ease, EaseFunction, EasingType};
use std::time::Duration;

const SCALE_NORMAL: Vec3 = Vec3::new(1., 1., 1.);
const SCALE_ENLARGED: Vec3 = Vec3::new(1.06, 1.06, 1.);

#[derive(Component)]
pub struct MainCamera;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Clone)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub entity: Entity,
    pub outer_hexagon: Entity,
    pub inner_hexagon: Entity,
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
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &ResMut<CellColors>,
    ) {
        // Normal scale
        self.rescale(commands, t, SCALE_NORMAL);
        // Set colors to normal
        self.set_colors(
            cell_colors.yellow_light.id,
            cell_colors.yellow_medium.id,
            color_query,
        );
    }

    pub fn uncover_number(
        &self,
        commands: &mut Commands,
        entity: Entity,
        t: &Transform,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
        number_cell: &NumberCell,
    ) {
        println!("Number");
        self.uncover(
            commands,
            entity,
            t,
            color_query,
            (cell_colors.gray_dark.id, cell_colors.gray_light.id),
        );
    }

    pub fn uncover_empty(
        &self,
        commands: &mut Commands,
        entity: Entity,
        t: &Transform,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        println!("Empty");
        self.uncover(
            commands,
            entity,
            t,
            color_query,
            (cell_colors.blue_dark.id, cell_colors.blue_light.id),
        );
    }

    fn uncover(
        &self,
        commands: &mut Commands,
        entity: Entity,
        t: &Transform,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        (dark, light): (HandleId, HandleId),
    ) {
        commands.entity(entity).remove_bundle::<HiddenCell>();
        // TODO: Uncover animation/particles
        // Normal scale
        self.rescale(commands, t, SCALE_NORMAL);
        self.set_colors(light, dark, color_query);
    }

    fn rescale(&self, commands: &mut Commands, orig: &Transform, scale: Vec3) {
        // Rescale hexagon to desired scale by easing
        let mut t1 = orig.clone();
        t1.scale = scale;
        commands.entity(self.entity).insert(orig.ease_to(
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

#[derive(Bundle)]
pub struct HiddenCell {
    // pub hoverable: Hoverable,
    pub clickable: Clickable,
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

pub struct SfxHover(pub Handle<AudioSource>);
