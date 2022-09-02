use super::functions::get_colors_for_app_state;
use crate::{
    components::Cell,
    functions::switch_state,
    resources::{GameColors, LoadState},
    states::AppState,
};
use bevy::{
    prelude::{Commands, Component, Handle, Query, State},
    sprite::ColorMaterial,
};

#[derive(Component)]
pub struct OptionCell {
    pub app_state: AppState,
}

impl OptionCell {
    pub fn hover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        game_colors: &GameColors,
    ) {
        let colors = get_colors_for_app_state(game_colors, self.app_state);
        cell.hover(commands, None, colors.1, colors.0, color_query);
    }

    pub fn unhover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        game_colors: &GameColors,
    ) {
        let colors = get_colors_for_app_state(game_colors, self.app_state);
        cell.unhover(commands, None, colors.2, colors.1, color_query);
    }

    pub fn click(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        game_colors: &GameColors,
        (app_state, load_state): (&mut State<AppState>, &mut LoadState),
    ) {
        if cell.hovering {
            cell.hovering = false;
        }
        let colors = get_colors_for_app_state(game_colors, self.app_state);
        cell.click(commands, None, colors.2, colors.1, color_query);
        // TODO: Remove
        // load_state.filename = Some("assets/levels/2/2.lvl".to_string());
        switch_state(Some(self.app_state), app_state, load_state);
    }
}
