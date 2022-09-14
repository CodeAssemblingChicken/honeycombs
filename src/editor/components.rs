use super::board::Board;
use crate::{components::Cell, enums::CellType, resources::GameColors};
use bevy::{
    prelude::{Commands, Component, Entity, EventWriter, Handle, Query},
    sprite::ColorMaterial,
};

#[derive(Component)]
pub struct EditorCell {
    pub hidden: bool,
    pub cell_type: Option<CellType>,
    pub text_entity: Entity,
}

impl EditorCell {
    pub fn hover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        game_colors: &GameColors,
    ) {
        let (light, dark) = if self.hidden {
            (
                game_colors.yellow_medium.clone(),
                game_colors.yellow_dark.clone(),
            )
        } else {
            match self.cell_type {
                Some(CellType::NumberCell(_)) => (
                    game_colors.gray_medium.clone(),
                    game_colors.gray_dark.clone(),
                ),
                Some(CellType::EmptyCell) => (
                    game_colors.blue_medium.clone(),
                    game_colors.blue_dark.clone(),
                ),
                None => (game_colors.alpha0.clone(), game_colors.alpha2.clone()),
            }
        };
        cell.hover(commands, None, light, dark, color_query);
    }

    pub fn unhover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        game_colors: &GameColors,
    ) {
        let (light, dark) = if let Some(ct) = self.cell_type {
            if self.hidden {
                (
                    game_colors.yellow_light.clone(),
                    game_colors.yellow_medium.clone(),
                )
            } else {
                match ct {
                    CellType::NumberCell(_) => (
                        game_colors.gray_light.clone(),
                        game_colors.gray_medium.clone(),
                    ),
                    CellType::EmptyCell => (
                        game_colors.blue_light.clone(),
                        game_colors.blue_medium.clone(),
                    ),
                }
            }
        } else {
            (game_colors.alpha0.clone(), game_colors.alpha1.clone())
        };
        cell.unhover(commands, None, light, dark, color_query);
    }

    pub fn toggle_hidden(
        &mut self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        game_colors: &GameColors,
        board: &mut Board,
        ev_cell_update: &mut EventWriter<CellUpdateEvent>,
    ) {
        if self.cell_type.is_none() {
            return;
        }
        self.hidden = !self.hidden;
        let (c1, c2) = if self.hidden {
            (
                game_colors.yellow_light.clone(),
                game_colors.yellow_medium.clone(),
            )
        } else {
            match self.cell_type.unwrap() {
                CellType::NumberCell(_) => (
                    game_colors.gray_light.clone(),
                    game_colors.gray_medium.clone(),
                ),
                CellType::EmptyCell => (
                    game_colors.blue_light.clone(),
                    game_colors.blue_medium.clone(),
                ),
            }
        };
        // TODO: Does it really make sense to click here?
        cell.click(commands, None, c1, c2, color_query);
        board.cells[cell.y as usize][cell.x as usize].1 = self.hidden;
        ev_cell_update.send(CellUpdateEvent);
    }
}

/// Component for the NumberCell type
#[derive(Debug, Component)]
pub struct NumberCell {
    pub count: u8,
    pub label: Entity,
    pub special_hint: bool,
}

/// Component for the EmptyCell type
#[derive(Debug, Component)]
pub struct EmptyCell;

#[derive(Debug, Component)]
pub struct UnsetCell;

pub struct CellUpdateEvent;
