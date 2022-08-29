use crate::{
    components::{Cell, CellType},
    resources::CellColors,
};
use bevy::{
    prelude::{Commands, Component, Entity, Handle, Query},
    sprite::ColorMaterial,
};

pub struct Board {
    pub cells: Vec<Vec<Option<CellType>>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![None; width]; height],
            width,
            height,
        }
    }
}

#[derive(Component)]
pub struct EditorCell {
    pub hidden: bool,
    pub cell_type: Option<CellType>,
}

impl EditorCell {
    pub fn hover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        let (light, dark) = if self.hidden {
            (
                cell_colors.yellow_medium.clone(),
                cell_colors.yellow_dark.clone(),
            )
        } else {
            match self.cell_type {
                Some(CellType::NumberCell(_)) => (
                    cell_colors.gray_medium.clone(),
                    cell_colors.gray_dark.clone(),
                ),
                Some(CellType::EmptyCell) => (
                    cell_colors.blue_medium.clone(),
                    cell_colors.blue_dark.clone(),
                ),
                None => (cell_colors.alpha0.clone(), cell_colors.alpha2.clone()),
            }
        };
        cell.hover(commands, None, light, dark, color_query);
    }

    pub fn unhover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        let (light, dark) = if let Some(ct) = self.cell_type {
            if self.hidden {
                (
                    cell_colors.yellow_light.clone(),
                    cell_colors.yellow_medium.clone(),
                )
            } else {
                match ct {
                    CellType::NumberCell(_) => (
                        cell_colors.gray_light.clone(),
                        cell_colors.gray_medium.clone(),
                    ),
                    CellType::EmptyCell => (
                        cell_colors.blue_light.clone(),
                        cell_colors.blue_medium.clone(),
                    ),
                }
            }
        } else {
            (cell_colors.alpha0.clone(), cell_colors.alpha1.clone())
        };
        // let (light, dark) = if self.hidden {
        //     (
        //         cell_colors.yellow_light.clone(),
        //         cell_colors.yellow_medium.clone(),
        //     )
        // } else {
        //     match self.cell_type {
        //         Some(CellType::NumberCell(_)) => (
        //             cell_colors.gray_light.clone(),
        //             cell_colors.gray_medium.clone(),
        //         ),
        //         Some(CellType::EmptyCell) => (
        //             cell_colors.blue_light.clone(),
        //             cell_colors.blue_medium.clone(),
        //         ),
        //         None =>
        //     }
        // };
        cell.unhover(commands, None, light, dark, color_query);
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
