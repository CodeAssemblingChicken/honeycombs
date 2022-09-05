use crate::constants::{SCALE_ENLARGED, SCALE_NORMAL};
use bevy::{
    math::Vec3,
    prelude::{Bundle, Color, Commands, Component, Entity, Handle, Query, Transform},
    sprite::ColorMaterial,
    text::{TextSection, TextStyle},
};
use bevy_easings::{Ease, EaseFunction, EasingType};
use interactable::{click::Clickable, hover::Hoverable};
use serde::Deserialize;
use std::time::Duration;

// TODO: This is probably way to big
/// Cell component storing everythin cell related
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
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
        background: Option<Handle<ColorMaterial>>,
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
        if !self.hovering {
            return;
        }
        self.hovering = false;
        // Normal scale
        self.rescale(commands, SCALE_NORMAL);
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
        self.rescale(commands, SCALE_NORMAL);
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

/// Only hidden cells are Hoverable and Clickable
#[derive(Bundle)]
pub struct InteractableCell {
    pub hoverable: Hoverable,
    pub clickable: Clickable,
}

/// Used for querying only the inner hexes
#[derive(Debug, Component)]
pub struct CellInner;
/// Used for querying only the outer hexes
#[derive(Debug, Component)]
pub struct CellOuter;

/// The type of cell.
/// Used in cell component for uncover-handling
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    NumberCell(HintType),
    EmptyCell,
}

/// Component for column hints
#[derive(Debug, Component, Clone, Copy)]
pub struct ColumnHint {
    pub x: usize,
    pub y: usize,
    pub dir: HintDirection,
    pub hint_type: HintType,
}

/// Direction of the column/row hints.
/// Straight down (TOP), down-right (RIGHT) and down-left (LEFT)
#[derive(Debug, Clone, Copy)]
pub enum HintDirection {
    Down,
    LeftDown,
    RightDown,
    Up,
    LeftUp,
    RightUp,
}

/// Indicator for special hints (connected or seperated cells)
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HintType {
    None,
    // SOME is quite ugly, it is used in parsing to indicate that the hint
    // is special and the concrete specialization (CONNECTED or SEPERATED)
    // must first be calculated
    // TODO: Think of something better
    Some,
    Connected,
    Seperated,
}

/// Required because of bevy_inspector_egui::Inspectable
impl Default for HintType {
    fn default() -> Self {
        Self::None
    }
}

/// Used to pass configuration from parser to board
pub struct BoardConfig {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<(Option<CellType>, bool)>>,
    pub hints: Vec<ColumnHint>,
    pub text: Option<(i32, i32, Vec<TextSectionConfig>)>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TextSectionConfig {
    pub text: String,
    pub color: Option<Color>,
    pub interactable: bool,
}

impl TextSectionConfig {
    pub fn new(text: impl Into<String>, color: Option<Color>, interactable: bool) -> Self {
        Self {
            text: text.into(),
            color,
            interactable,
        }
    }
    pub fn to_text_section(&self, text_style: &TextStyle) -> TextSection {
        let mut ts = text_style.clone();
        if let Some(color) = self.color {
            ts.color = color;
        }
        TextSection::new(self.text.clone(), ts)
    }
}

#[derive(Component)]
pub struct RootComponent;
