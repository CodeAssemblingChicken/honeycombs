use crate::{
    components::Cell,
    functions::rescale_board,
    resources::{GameColors, LoadState, Profile, SfxHover},
    states::AppState,
};
use bevy::{
    audio::{Audio, PlaybackSettings},
    prelude::{Camera, Commands, EventReader, Handle, Query, Res, ResMut, State, Transform, With},
    sprite::ColorMaterial,
    window::WindowResized,
};
use interactable::{
    click::{ClickType, MouseLeftClickEvent},
    hover::{MouseEnterEvent, MouseExitEvent},
};

use super::components::OptionCell;

pub fn mouse_click_cell(
    mut commands: Commands,
    mut option_cell_query: Query<(&OptionCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    (mut app_state, mut load_state): (ResMut<State<AppState>>, ResMut<LoadState>),
    mut ev_mouse_left_click: EventReader<MouseLeftClickEvent>,
) {
    for ev in ev_mouse_left_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((oc, mut cell)) = option_cell_query.get_mut(ev.entity) {
            oc.click(
                &mut cell,
                &mut commands,
                &mut color_query,
                &game_colors,
                (&mut app_state, &mut load_state),
            );
        }
    }
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut option_cell_query: Query<(&OptionCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    (game_colors, profile): (Res<GameColors>, Res<Profile>),
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
    audio: Res<Audio>,
    clip: Res<SfxHover>,
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok((oc, mut cell)) = option_cell_query.get_mut(ev.0) {
            audio.play_with_settings(
                clip.0.clone(),
                PlaybackSettings::ONCE.with_volume(profile.sfx_volume),
            );
            oc.hover(&mut cell, &mut commands, &mut color_query, &game_colors);
        }
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut option_cell_query: Query<(&OptionCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok((oc, mut cell)) = option_cell_query.get_mut(ev.0) {
            oc.unhover(&mut cell, &mut commands, &mut color_query, &game_colors);
        }
    }
}

/// On resizing the window, the board is resized too
/// i.e. the camera zoom (scale) is recalculated
pub fn window_resize_system(
    mut ev_window_resize: EventReader<WindowResized>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for ev in ev_window_resize.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(10, 6, 1, ev.width, ev.height, &mut camera_query);
    }
}