use super::board::Board;
use crate::{components::Cell, constants::RADIUS, enums::CellType, resources::GameColors};
use bevy::{
    math::Vec3,
    prelude::{ColorMaterial, Commands, Component, Entity, Handle, Query, Visibility},
};
use bevy_easings::{Ease, EaseFunction, EasingType};
use std::time::Duration;

#[derive(Component)]
pub struct GameCell {
    pub cell_type: CellType,
    pub hidden: bool,
}

impl GameCell {
    /// Called when cell is hidden and the mouse enters it
    pub fn hover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        game_colors: &GameColors,
    ) {
        if !self.hidden {
            return;
        }
        // Pass event to Cell component with yellow colors
        cell.hover(
            commands,
            None,
            game_colors.yellow_medium.clone(),
            game_colors.yellow_dark.clone(),
            color_query,
        );
    }

    /// Called when cell is hidden and the mouse exits it
    pub fn unhover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        game_colors: &GameColors,
    ) {
        if !self.hidden {
            return;
        }
        cell.unhover(
            commands,
            None,
            game_colors.yellow_light.clone(),
            game_colors.yellow_medium.clone(),
            color_query,
        );
    }

    /// Called when cell is hidden and clicked on with the correct mouse button
    pub fn uncover(
        &mut self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        game_colors: &GameColors,
        number_cell: Option<&NumberCell>,
        board: &mut Board,
    ) {
        self.hidden = false;
        // TODO: Uncover animation/particles
        let (dark, light) = match self.cell_type {
            CellType::NumberCell(_) => {
                board.uncover_number();
                commands
                    .entity(number_cell.unwrap().label)
                    .remove::<Visibility>()
                    .insert(Visibility { is_visible: true });
                (
                    game_colors.gray_medium.clone(),
                    game_colors.gray_light.clone(),
                )
            }
            CellType::EmptyCell => {
                board.uncover_empty();
                (
                    game_colors.blue_medium.clone(),
                    game_colors.blue_light.clone(),
                )
            }
        };
        // TODO: Could break
        // commands.entity(cell.entity).remove::<Interactable>();
        interactable::remove_interactable(commands, cell.entity);
        // Normal scale
        cell.click(commands, None, light, dark, color_query);
    }

    /// Called when cell is hidden and clicked on with the wrong mouse button
    pub fn uncover_fail(&self, cell: &Cell, commands: &mut Commands, board: &mut Board) {
        let mut t1 = cell.orig;
        let mut t2 = cell.orig;
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
        board.make_mistake();
    }
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

#[derive(Debug, Component)]
pub struct RemainingText;

#[derive(Debug, Component)]
pub struct MistakesText;
