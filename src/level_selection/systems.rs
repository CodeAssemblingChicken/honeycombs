use super::components::LevelSelectionCell;
use crate::{
    components::{Cell, RootComponent},
    functions::{rescale_board, switch_state},
    resources::{GameColors, LoadState, Profile, SfxAssets},
    states::AppState,
};
use bevy::{
    input::Input,
    prelude::{Commands, EventReader, Handle, KeyCode, Query, Res, ResMut, State, Transform, With},
    sprite::ColorMaterial,
    window::WindowResized,
};
use bevy_kira_audio::{Audio, AudioControl};
use interactable::components::{Entered, Exited, ReleasedLeft};

pub fn mouse_click_cell(
    mut commands: Commands,
    mut level_cell_query: Query<(&LevelSelectionCell, &mut Cell), With<ReleasedLeft>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    (mut app_state, mut load_state): (ResMut<State<AppState>>, ResMut<LoadState>),
) {
    for (lsc, mut cell) in level_cell_query.iter_mut() {
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

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut level_cell_query: Query<(&LevelSelectionCell, &mut Cell), With<Entered>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    (game_colors, profile): (Res<GameColors>, Res<Profile>),
    audio: Res<Audio>,
    sfx_assets: Res<SfxAssets>,
) {
    for (lsc, mut cell) in level_cell_query.iter_mut() {
        audio
            .play(sfx_assets.sfx_hover.clone())
            .with_volume(profile.sfx_volume as f64);
        lsc.hover(
            &mut cell,
            &mut commands,
            &mut color_query,
            &game_colors,
            &profile,
        );
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut level_cell_query: Query<(&LevelSelectionCell, &mut Cell), With<Exited>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    profile: Res<Profile>,
) {
    for (lsc, mut cell) in level_cell_query.iter_mut() {
        lsc.unhover(
            &mut cell,
            &mut commands,
            &mut color_query,
            &game_colors,
            &profile,
        );
    }
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
