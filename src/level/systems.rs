use super::{
    board::Board,
    components::{EmptyCell, GameCell, NumberCell},
};
use crate::{
    components::Cell,
    functions::{rescale_board, switch_state},
    resources::{GameColors, LoadState, Profile, SfxHover},
    states::AppState,
};
use bevy::{
    audio::{Audio, PlaybackSettings},
    input::Input,
    prelude::{
        Camera, ColorMaterial, Commands, EventReader, Handle, KeyCode, Query, Res, ResMut, State,
        Transform, With, Without,
    },
    window::WindowResized,
};
use interactable::{
    click::{ClickType, MouseLeftClickEvent, MouseRightClickEvent},
    hover::{MouseEnterEvent, MouseExitEvent, MouseOverEvent},
};

/// Calls uncover on a cell that is clicked by the mouse
pub fn mouse_click_cell(
    mut commands: Commands,
    mut number_cell_query: Query<(&mut GameCell, &mut Cell, &NumberCell), Without<EmptyCell>>,
    mut empty_cell_query: Query<(&mut GameCell, &mut Cell), With<EmptyCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    mut board: ResMut<Board>,
    (mut ev_mouse_left_click, mut ev_mouse_right_click): (
        EventReader<MouseLeftClickEvent>,
        EventReader<MouseRightClickEvent>,
    ),
) {
    for ev in ev_mouse_left_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((lc, cell, _nc)) = number_cell_query.get(ev.entity) {
            lc.uncover_fail(cell, &mut commands);
        }
        if let Ok((mut lc, mut cell)) = empty_cell_query.get_mut(ev.entity) {
            lc.uncover(
                &mut cell,
                &mut commands,
                &mut color_query,
                game_colors.as_ref(),
                None,
                &mut board,
            );
        }
    }
    for ev in ev_mouse_right_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((mut lc, mut cell, nc)) = number_cell_query.get_mut(ev.entity) {
            lc.uncover(
                &mut cell,
                &mut commands,
                &mut color_query,
                game_colors.as_ref(),
                Some(nc),
                &mut board,
            );
        }
        if let Ok((lc, cell)) = empty_cell_query.get(ev.entity) {
            lc.uncover_fail(cell, &mut commands);
        }
    }
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut cell_query: Query<(&GameCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
    (audio, clip, profile): (Res<Audio>, Res<SfxHover>, Res<Profile>),
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok((lc, mut cell)) = cell_query.get_mut(ev.0) {
            audio.play_with_settings(
                clip.0.clone(),
                PlaybackSettings::ONCE.with_volume(profile.sfx_volume),
            );
            lc.hover(&mut cell, &mut commands, &mut color_query, &game_colors);
        }
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut cell_query: Query<(&GameCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok((lc, mut cell)) = cell_query.get_mut(ev.0) {
            lc.unhover(&mut cell, &mut commands, &mut color_query, &game_colors);
        }
    }
}

// TODO: Not used
/// Could call a function on the currently hovered cell, but doesn't right now
#[allow(unused_mut, unused_variables)]
pub fn mouse_over_cell(
    mut commands: Commands,
    cell_query: Query<&Transform, With<Cell>>,
    mut ev_mouse_over: EventReader<MouseOverEvent>,
) {
    for ev in ev_mouse_over.iter() {}
}

/// On resizing the window, the board is resized too
/// i.e. the camera zoom (scale) is recalculated
pub fn window_resize_system(
    mut ev_window_resize: EventReader<WindowResized>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    board: Res<Board>,
) {
    for ev in ev_window_resize.iter() {
        rescale_board(
            board.width,
            board.height,
            4,
            ev.width,
            ev.height,
            &mut camera_query,
        );
    }
}

pub fn hotkey_system(
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut load_state: ResMut<LoadState>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        switch_state(
            Some(AppState::LevelSelection),
            &mut app_state,
            &mut load_state,
        );
    }
}

pub fn check_solved(
    board: Res<Board>,
    mut app_state: ResMut<State<AppState>>,
    mut load_state: ResMut<LoadState>,
) {
    if board.is_changed() && board.remaining == 0 {
        switch_state(
            Some(AppState::LevelSelection),
            &mut app_state,
            &mut load_state,
        );
    }
}
