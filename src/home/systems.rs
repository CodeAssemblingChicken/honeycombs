use crate::{
    components::{Cell, RootComponent},
    functions::{rescale_board, switch_state},
    resources::{GameColors, LoadState, Locale, Profile, SfxAssets},
    states::AppState,
};
use bevy::{
    audio::{Audio, PlaybackSettings},
    prelude::{Commands, EventReader, Handle, Query, Res, ResMut, State, Transform, With},
    sprite::ColorMaterial,
    window::WindowResized,
};
use interactable::{
    click::{ClickType, MouseLeftClickEvent},
    hover::{MouseEnterEvent, MouseExitEvent},
};

use super::components::{Language, OptionCell};

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

pub fn mouse_click_lang(
    level_cell_query: Query<&Language>,
    (mut app_state, mut load_state, mut locale, mut profile): (
        ResMut<State<AppState>>,
        ResMut<LoadState>,
        ResMut<Locale>,
        ResMut<Profile>,
    ),
    mut ev_mouse_left_click: EventReader<MouseLeftClickEvent>,
) {
    for ev in ev_mouse_left_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok(lang) = level_cell_query.get(ev.entity) {
            locale.set_lang(
                match lang {
                    Language::EN => "en",
                    Language::DE => "de",
                    Language::FR => "fr",
                    Language::ES => "es",
                },
                &mut profile,
            );
            switch_state(Some(AppState::Home), &mut app_state, &mut load_state);
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
    sfx_assets: Res<SfxAssets>,
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok((oc, mut cell)) = option_cell_query.get_mut(ev.0) {
            audio.play_with_settings(
                sfx_assets.sfx_hover.clone(),
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
