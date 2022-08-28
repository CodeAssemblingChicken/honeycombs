use bevy::{
    prelude::{Commands, Component, Entity, Handle, Query},
    sprite::ColorMaterial,
};

use crate::{
    components::{Cell, CellType},
    resources::CellColors,
};

#[derive(Component)]
pub struct EditorCell {
    pub cell_type: Option<CellType>,
}

impl EditorCell {
    /// Called when cell is hidden and the mouse enters it
    pub fn hover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        let (light, dark) = match self.cell_type {
            Some(CellType::NumberCell(_)) => (
                cell_colors.gray_medium.clone(),
                cell_colors.gray_dark.clone(),
            ),
            Some(CellType::EmptyCell) => (
                cell_colors.blue_medium.clone(),
                cell_colors.blue_dark.clone(),
            ),
            None => (cell_colors.alpha0.clone(), cell_colors.alpha2.clone()),
        };
        // Pass event to Cell component with yellow colors
        cell.hover(commands, None, light, dark, color_query);
    }

    /// Called when cell is hidden and the mouse exits it
    pub fn unhover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        let (light, dark) = match self.cell_type {
            Some(CellType::NumberCell(_)) => (
                cell_colors.gray_light.clone(),
                cell_colors.gray_medium.clone(),
            ),
            Some(CellType::EmptyCell) => (
                cell_colors.blue_light.clone(),
                cell_colors.blue_medium.clone(),
            ),
            None => (cell_colors.alpha0.clone(), cell_colors.alpha1.clone()),
        };
        // Pass event to Cell component with yellow colors
        cell.unhover(commands, None, light, dark, color_query);
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
pub struct UnsetCell;
