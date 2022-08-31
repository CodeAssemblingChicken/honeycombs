use crate::{
    components::Cell,
    resources::{CellColors, LevelFile},
    states::AppState,
};
use bevy::{
    prelude::{Commands, Component, Handle, Query, ResMut, State},
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
            None,
            cell_colors.blue_medium.clone(),
            cell_colors.blue_dark.clone(),
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
            None,
            cell_colors.blue_light.clone(),
            cell_colors.blue_medium.clone(),
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
            None,
            cell_colors.blue_light.clone(),
            cell_colors.blue_medium.clone(),
            color_query,
        );
        level_file.filename = Some(format!("assets/levels/{}/{}.lvl", self.stage, self.level));
        app_state.set(AppState::Level).unwrap();
    }
}

pub struct StageCluster {
    pub stage_no: u8,
    pub unlock_required: u32,
    pub num_levels: u8,
}

impl StageCluster {
    pub fn new(stage_no: u8, unlock_required: u32, num_levels: u8) -> Self {
        assert!(num_levels > 0 && num_levels <= 6);
        Self {
            stage_no,
            unlock_required,
            num_levels,
        }
    }
}

#[derive(Component)]
pub struct OptionCell {
    pub app_state: AppState,
}

impl OptionCell {
    pub fn click(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
        app_state: &mut State<AppState>,
        level_file: &mut LevelFile,
    ) {
        if cell.hovering {
            cell.hovering = false;
        }
        cell.click(
            commands,
            None,
            cell_colors.gray_light.clone(),
            cell_colors.gray_medium.clone(),
            color_query,
        );
        // TODO: Remove
        level_file.filename = Some("assets/levels/1/1.lvl".to_string());
        app_state.set(self.app_state).unwrap();
    }
}
