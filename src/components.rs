use crate::{functions::spawn_cell_text, RADIUS, SCALE_ENLARGED, SCALE_NORMAL};
use bevy::{
    asset::HandleId,
    audio::AudioSource,
    math::Vec3,
    prelude::{
        Bundle, Color, ColorMaterial, Commands, Component, Entity, Handle, Query, ResMut, Transform,
    },
    text::{TextAlignment, TextStyle},
};
use bevy_easings::{Ease, EaseFunction, EasingType};
use interactable::{click::Clickable, hover::Hoverable};
use std::time::Duration;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Clone)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub cell_type: CellType,
    pub entity: Entity,
    pub outer_hexagon: Entity,
    pub inner_hexagon: Entity,
    pub orig: Transform,
    pub hovering: bool,
}

impl Cell {
    pub fn hover(
        &mut self,
        commands: &mut Commands,
        child_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &ResMut<CellColors>,
    ) {
        if self.hovering {
            return;
        }
        self.hovering = true;
        // Enlarge
        self.rescale(commands, SCALE_ENLARGED);
        // Set colors to hovering
        self.set_colors(
            cell_colors.yellow_medium.id,
            cell_colors.yellow_dark.id,
            child_query,
        );
    }

    pub fn unhover(
        &mut self,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &ResMut<CellColors>,
    ) {
        if !self.hovering {
            return;
        }
        self.hovering = false;
        // Normal scale
        self.rescale(commands, SCALE_NORMAL);
        // Set colors to normal
        self.set_colors(
            cell_colors.yellow_light.id,
            cell_colors.yellow_medium.id,
            color_query,
        );
    }

    pub fn uncover(
        &mut self,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
        number_cell: Option<&NumberCell>,
        text_settings: &TextSettings,
    ) {
        // TODO: Uncover animation/particles
        if self.hovering {
            self.hovering = false;
        }
        let (dark, light) = match self.cell_type {
            CellType::NumberCell(ht) => {
                let mut ts = text_settings.clone();
                match ht {
                    HintType::CONNECTED => ts.style.color = Color::GREEN,
                    HintType::SEPERATED => ts.style.color = Color::RED,
                    _ => (),
                }
                spawn_cell_text(self.orig, commands, number_cell.unwrap(), &ts);
                (cell_colors.gray_dark.id, cell_colors.gray_light.id)
            }
            CellType::EmptyCell => (cell_colors.blue_dark.id, cell_colors.blue_light.id),
        };

        commands.entity(self.entity).remove_bundle::<HiddenCell>();
        // Normal scale
        self.rescale(commands, SCALE_NORMAL);
        self.set_colors(light, dark, color_query);
    }

    pub fn uncover_fail(&self, commands: &mut Commands) {
        let mut t1 = self.orig.clone();
        let mut t2 = self.orig.clone();
        t1.translation += Vec3::new(-RADIUS / 10., -RADIUS / 20., 0.0);
        t2.translation += Vec3::new(RADIUS / 15., RADIUS / 25., 0.0);
        commands.entity(self.entity).insert(
            self.orig
                .ease_to(
                    t1,
                    EaseFunction::BounceInOut,
                    EasingType::Once {
                        duration: Duration::from_millis(30),
                    },
                )
                .ease_to(
                    t2,
                    EaseFunction::BounceInOut,
                    EasingType::Once {
                        duration: Duration::from_millis(60),
                    },
                )
                .ease_to(
                    self.orig,
                    EaseFunction::BounceInOut,
                    EasingType::Once {
                        duration: Duration::from_millis(40),
                    },
                ),
        );
    }

    fn rescale(&self, commands: &mut Commands, scale: Vec3) {
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
    pub hoverable: Hoverable,
    pub clickable: Clickable,
}

#[derive(Debug, Component)]
pub struct NumberCell {
    pub count: u8,
    pub hint_type: HintType,
}

#[derive(Debug, Component)]
pub struct EmptyCell;

#[derive(Debug, Component)]
pub struct CellInner;
#[derive(Debug, Component)]
pub struct CellOuter;

#[derive(Debug, Component)]
pub struct ColumnHint {
    pub x: usize,
    pub y: usize,
    pub dir: HintDirection,
}

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

#[derive(Clone)]
pub struct TextSettings {
    pub style: TextStyle,
    pub alignment: TextAlignment,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellType {
    NumberCell(HintType),
    EmptyCell,
}

#[derive(Debug)]
pub enum HintDirection {
    TOP,
    LEFT,
    RIGHT,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HintType {
    NONE,
    SOME,
    CONNECTED,
    SEPERATED,
}

impl Default for HintType {
    fn default() -> Self {
        Self::NONE
    }
}
