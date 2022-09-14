use super::components::OptionCell;
use crate::{
    components::{Cell, Language, RootComponent},
    functions::{rescale_board, switch_state},
    resources::{GameColors, LoadState, Profile, SfxAssets},
    states::AppState,
};
use bevy::{
    prelude::{Commands, EventReader, Handle, Query, Res, ResMut, State, Transform, With},
    sprite::ColorMaterial,
    window::WindowResized,
};
use bevy_kira_audio::{Audio, AudioControl};
use interactable::components::{Entered, Exited, ReleasedLeft};

pub fn mouse_click_cell(
    mut commands: Commands,
    mut option_cell_query: Query<(&OptionCell, &mut Cell), With<ReleasedLeft>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    (mut app_state, mut load_state): (ResMut<State<AppState>>, ResMut<LoadState>),
) {
    for (oc, mut cell) in option_cell_query.iter_mut() {
        oc.click(
            &mut cell,
            &mut commands,
            &mut color_query,
            &game_colors,
            (&mut app_state, &mut load_state),
        );
    }
}

pub fn mouse_click_lang(
    level_cell_query: Query<&Language, With<ReleasedLeft>>,
    (mut app_state, mut load_state, mut profile): (
        ResMut<State<AppState>>,
        ResMut<LoadState>,
        ResMut<Profile>,
    ),
) {
    for lang in level_cell_query.iter() {
        profile.lang = *lang;
        switch_state(Some(AppState::Home), &mut app_state, &mut load_state);
    }
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut option_cell_query: Query<(&OptionCell, &mut Cell), With<Entered>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    (game_colors, profile): (Res<GameColors>, Res<Profile>),
    audio: Res<Audio>,
    sfx_assets: Res<SfxAssets>,
) {
    for (oc, mut cell) in option_cell_query.iter_mut() {
        audio
            .play(sfx_assets.sfx_hover.clone())
            .with_volume(profile.sfx_volume as f64);
        oc.hover(&mut cell, &mut commands, &mut color_query, &game_colors);
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut option_cell_query: Query<(&OptionCell, &mut Cell), With<Exited>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
) {
    for (oc, mut cell) in option_cell_query.iter_mut() {
        oc.unhover(&mut cell, &mut commands, &mut color_query, &game_colors);
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
            rescale_board(10, 6, 1, ev.width, ev.height, &mut root);
        }
    }
}
