use super::board::Board;
use crate::{
    components::{Cell, CellType},
    constants::RADIUS,
    resources::CellColors,
};
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
pub struct GameCell {
    pub cell_type: CellType,
}

impl GameCell {
    /// Called when cell is hidden and the mouse enters it
    pub fn hover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        // Pass event to Cell component with yellow colors
        cell.hover(
            commands,
            cell_colors.yellow_medium.clone(),
            cell_colors.yellow_dark.clone(),
            color_query,
        );
    }

    /// Called when cell is hidden and the mouse exits it
    pub fn unhover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        cell.unhover(
            commands,
            cell_colors.yellow_light.clone(),
            cell_colors.yellow_medium.clone(),
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
        board: &mut ResMut<Board>,
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
                (
                    cell_colors.gray_medium.clone(),
                    cell_colors.gray_light.clone(),
                )
            }
            CellType::EmptyCell => {
                board.remaining -= 1;
                (
                    cell_colors.blue_medium.clone(),
                    cell_colors.blue_light.clone(),
                )
            }
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
