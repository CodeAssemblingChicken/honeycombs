use crate::{
    components::Cell,
    resources::{GameColors, LoadState, Profile},
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
        game_colors: &GameColors,
        profile: &Profile,
    ) {
        let (c1, c2) = if profile.level_points[self.stage as usize][self.level as usize].is_some() {
            (
                game_colors.blue_medium.clone(),
                game_colors.blue_dark.clone(),
            )
        } else {
            (
                game_colors.yellow_medium.clone(),
                game_colors.yellow_dark.clone(),
            )
        };
        cell.hover(commands, None, c1, c2, color_query);
    }

    pub fn unhover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        game_colors: &GameColors,
        profile: &Profile,
    ) {
        let (c1, c2) = if profile.level_points[self.stage as usize][self.level as usize].is_some() {
            (
                game_colors.blue_light.clone(),
                game_colors.blue_medium.clone(),
            )
        } else {
            (
                game_colors.yellow_light.clone(),
                game_colors.yellow_medium.clone(),
            )
        };
        cell.unhover(commands, None, c1, c2, color_query);
    }

    pub fn click(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        game_colors: &GameColors,
        app_state: &mut ResMut<State<AppState>>,
        load_state: &mut ResMut<LoadState>,
    ) {
        cell.click(
            commands,
            None,
            game_colors.blue_light.clone(),
            game_colors.blue_medium.clone(),
            color_query,
        );
        load_state.filename = Some(format!(
            "assets/levels/{}/{}.lvl",
            self.stage + 1,
            self.level + 1
        ));
        load_state.ids = Some((self.stage, self.level));
        app_state.set(AppState::Level).unwrap();
    }
}

pub struct StageCluster {
    pub stage_no: u8,
    pub unlock_required: u16,
    pub num_levels: u8,
}

impl StageCluster {
    pub fn new(stage_no: u8, unlock_required: u16, num_levels: u8) -> Self {
        assert!(num_levels > 0 && num_levels <= 6);
        Self {
            stage_no,
            unlock_required,
            num_levels,
        }
    }
}
