use crate::{components::Cell, resources::CellColors, RADIUS};
use bevy::{
    math::Vec3,
    prelude::{
        Bundle, ColorMaterial, Commands, Component, Entity, Handle, Query, ResMut, Visibility,
    },
};
use bevy_easings::{Ease, EaseFunction, EasingType};
use interactable::{click::Clickable, hover::Hoverable};
use std::time::Duration;

#[derive(Component)]
pub struct LevelCell {
    pub cell_type: CellType,
}

impl LevelCell {
    /// Called when cell is hidden and the mouse enters it
    pub fn hover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &ResMut<CellColors>,
    ) {
        // Pass event to Cell component with yellow colors
        cell.hover(
            commands,
            cell_colors.yellow_medium.id,
            cell_colors.yellow_dark.id,
            color_query,
        );
    }

    /// Called when cell is hidden and the mouse exits it
    pub fn unhover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &ResMut<CellColors>,
    ) {
        cell.unhover(
            commands,
            cell_colors.yellow_light.id,
            cell_colors.yellow_medium.id,
            color_query,
        );
    }

    /// Called when cell is hidden and clicked on with the correct mouse button
    pub fn uncover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
        number_cell: Option<&NumberCell>,
    ) {
        // TODO: Uncover animation/particles
        if cell.hovering {
            cell.hovering = false;
        }
        let (dark, light) = match self.cell_type {
            CellType::NumberCell(_) => {
                commands
                    .entity(number_cell.unwrap().label)
                    .remove::<Visibility>()
                    .insert(Visibility { is_visible: true });
                (cell_colors.gray_medium.id, cell_colors.gray_light.id)
            }
            CellType::EmptyCell => (cell_colors.blue_medium.id, cell_colors.blue_light.id),
        };

        commands.entity(cell.entity).remove_bundle::<HiddenCell>();
        // Normal scale
        cell.click(commands, light, dark, color_query);
    }

    /// Called when cell is hidden and clicked on with the wrong mouse button
    pub fn uncover_fail(&self, cell: &Cell, commands: &mut Commands) {
        let mut t1 = cell.orig.clone();
        let mut t2 = cell.orig.clone();
        t1.translation += Vec3::new(-RADIUS / 10., -RADIUS / 20., 0.0);
        t2.translation += Vec3::new(RADIUS / 15., RADIUS / 25., 0.0);
        commands.entity(cell.entity).insert(
            cell.orig
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
                    cell.orig,
                    EaseFunction::BounceInOut,
                    EasingType::Once {
                        duration: Duration::from_millis(40),
                    },
                ),
        );
    }
}

/// Only hidden cells are Hoverable and Clickable
#[derive(Bundle)]
pub struct HiddenCell {
    pub hoverable: Hoverable,
    pub clickable: Clickable,
}

/// Component for the NumberCell type
#[derive(Debug, Component)]
pub struct NumberCell {
    pub count: u8,
    pub label: Entity,
}

/// Component for the EmptyCell type
#[derive(Debug, Component)]
pub struct EmptyCell;

/// Used for querying only the inner hexes
#[derive(Debug, Component)]
pub struct CellInner;
/// Used for querying only the outer hexes
#[derive(Debug, Component)]
pub struct CellOuter;

/// Component for column hints
#[derive(Debug, Component)]
pub struct ColumnHint {
    pub x: usize,
    pub y: usize,
    pub dir: HintDirection,
    pub hint_type: HintType,
}

/// The type of cell.
/// Used in cell component for uncover-handling
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellType {
    NumberCell(HintType),
    EmptyCell,
}

/// Direction of the column/row hints.
/// Straight down (TOP), down-right (RIGHT) and down-left (LEFT)
#[derive(Debug)]
pub enum HintDirection {
    TOP,
    LEFT,
    RIGHT,
}

/// Indicator for special hints (connected or seperated cells)
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HintType {
    NONE,
    // SOME is quite ugly, it is used in parsing to indicate that the hint
    // is special and the concrete specialization (CONNECTED or SEPERATED)
    // must first be calculated
    // TODO: Think of something better
    SOME,
    CONNECTED,
    SEPERATED,
}

/// Required because of bevy_inspector_egui::Inspectable
impl Default for HintType {
    fn default() -> Self {
        Self::NONE
    }
}
