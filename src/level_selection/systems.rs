use super::components::LevelSelectionCell;
use crate::{
    components::{Cell, RootComponent},
    functions::{rescale_board, switch_state},
    resources::{GameColors, LoadState, Profile, SfxAssets},
    states::AppState,
};
use bevy::{
    audio::{Audio, PlaybackSettings},
    input::Input,
    prelude::{Commands, EventReader, Handle, KeyCode, Query, Res, ResMut, State, Transform, With},
    sprite::ColorMaterial,
    window::WindowResized,
};
use interactable::{
    click::{ClickType, MouseLeftClickEvent},
    hover::{MouseEnterEvent, MouseExitEvent, MouseOverEvent},
};

pub fn mouse_click_cell(
    mut commands: Commands,
    mut level_cell_query: Query<(&LevelSelectionCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    (mut app_state, mut load_state): (ResMut<State<AppState>>, ResMut<LoadState>),
    mut ev_mouse_left_click: EventReader<MouseLeftClickEvent>,
) {
    for ev in ev_mouse_left_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((lsc, mut cell)) = level_cell_query.get_mut(ev.entity) {
            lsc.click(
                &mut cell,
                &mut commands,
                &mut color_query,
                &game_colors,
                &mut app_state,
                &mut load_state,
            );
        }
    }
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut level_cell_query: Query<(&LevelSelectionCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    (game_colors, profile): (Res<GameColors>, Res<Profile>),
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
    audio: Res<Audio>,
    sfx_assets: Res<SfxAssets>,
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok((lsc, mut cell)) = level_cell_query.get_mut(ev.0) {
            audio.play_with_settings(
                sfx_assets.sfx_hover.clone(),
                PlaybackSettings::ONCE.with_volume(profile.sfx_volume),
            );
            lsc.hover(
                &mut cell,
                &mut commands,
                &mut color_query,
                &game_colors,
                &profile,
            );
        }
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut level_cell_query: Query<(&LevelSelectionCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    profile: Res<Profile>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok((lsc, mut cell)) = level_cell_query.get_mut(ev.0) {
            lsc.unhover(
                &mut cell,
                &mut commands,
                &mut color_query,
                &game_colors,
                &profile,
            );
        }
    }
}

// TODO: Not used
/// Could call a function on the currently hovered cell, but doesn't right now
#[allow(unused_mut, unused_variables)]
pub fn mouse_over_cell(
    mut commands: Commands,
    mut cell_query: Query<&mut Cell>,
    mut ev_mouse_over: EventReader<MouseOverEvent>,
) {
    for ev in ev_mouse_over.iter() {}
}

pub fn hotkey_system(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut load_state: ResMut<LoadState>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        keys.clear_just_pressed(KeyCode::Escape);
        switch_state(Some(AppState::Home), &mut app_state, &mut load_state);
    }
}

/// On resizing the window, the board is resized too
/// i.e. the camera zoom (scale) is recalculated
pub fn window_resize_system(
    mut ev_window_resize: EventReader<WindowResized>,
    mut root_query: Query<&mut Transform, With<RootComponent>>,
) {
    for ev in ev_window_resize.iter() {
        if let Ok(mut root) = root_query.get_single_mut() {
            // TODO: Remove hard-coded width/height
            rescale_board(11, 11, 1, ev.width, ev.height, &mut root);
        }
    }
}
