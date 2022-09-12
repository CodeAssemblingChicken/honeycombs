use crate::{
    components::Cell,
    constants::{MED_SCALE, RADIUS},
    functions::{make_cell_interactable, spawn_cell, spawn_cell_text},
    resources::{CellMeshes, GameColors, TextSettings},
    states::AppState,
};
use bevy::{
    hierarchy::BuildChildren,
    prelude::{Commands, Entity, Handle, Transform},
    sprite::ColorMaterial,
};

use super::components::OptionCell;

pub fn spawn_option_cell(
    commands: &mut Commands,
    cell_meshes: &CellMeshes,
    game_colors: &GameColors,
    text_settings: &TextSettings,
    big_transform: Transform,
    app_state: AppState,
    text: &str,
) -> Entity {
    let cell = commands.spawn().id();
    let colors = get_colors_for_app_state(game_colors, app_state);
    let (child1, child2) = spawn_cell(
        commands,
        cell,
        (
            cell_meshes.med_hexagon_back.clone(),
            cell_meshes.med_hexagon_outer.clone(),
            cell_meshes.med_hexagon_inner.clone(),
        ),
        (game_colors.white.clone(), colors.1, colors.2),
        big_transform,
    );

    let text_entity = spawn_cell_text(
        commands,
        text,
        text_settings.style_cell_large.clone(),
        text_settings.alignment,
    );
    commands.entity(cell).add_child(text_entity);

    make_cell_interactable(commands, cell, RADIUS * MED_SCALE);

    let cell_component = Cell {
        x: -1,
        y: -1,
        entity: cell,
        outer_hexagon: child1,
        inner_hexagon: child2,
        orig: big_transform,
    };
    commands
        .entity(cell)
        .insert(cell_component)
        .insert(OptionCell { app_state });

    cell
}

pub fn get_colors_for_app_state(
    game_colors: &GameColors,
    app_state: AppState,
) -> (
    Handle<ColorMaterial>,
    Handle<ColorMaterial>,
    Handle<ColorMaterial>,
) {
    match app_state {
        AppState::LevelSelection => (
            game_colors.blue_dark.clone(),
            game_colors.blue_medium.clone(),
            game_colors.blue_light.clone(),
        ),
        AppState::Editor => (
            game_colors.yellow_dark.clone(),
            game_colors.yellow_medium.clone(),
            game_colors.yellow_light.clone(),
        ),
        _ => (
            game_colors.gray_dark.clone(),
            game_colors.gray_medium.clone(),
            game_colors.gray_light.clone(),
        ),
    }
}
