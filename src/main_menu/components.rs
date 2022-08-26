use crate::{
    components::Cell, level::resources::LevelFile, resources::CellColors, states::AppState,
};
use bevy::{
    prelude::{Commands, Component, Handle, Query, Res, ResMut, State},
    sprite::ColorMaterial,
};

#[derive(Component)]
pub struct LevelSelectionCell {
    pub stage: u8,
    pub level: u8,
}

impl LevelSelectionCell {
    pub fn hover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        cell.hover(
            commands,
            cell_colors.blue_medium.id,
            cell_colors.blue_dark.id,
            color_query,
        );
    }

    pub fn unhover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        cell.unhover(
            commands,
            cell_colors.blue_light.id,
            cell_colors.blue_medium.id,
            color_query,
        );
    }

    pub fn click(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
        app_state: &mut ResMut<State<AppState>>,
        level_file: &mut ResMut<LevelFile>,
    ) {
        if cell.hovering {
            cell.hovering = false;
        }
        cell.click(
            commands,
            cell_colors.blue_light.id,
            cell_colors.blue_medium.id,
            color_query,
        );
        level_file.filename = Some(format!("assets/levels/{}/{}.lvl", self.stage, self.level));
        app_state.set(AppState::Level).unwrap();
    }
}
