use super::{
    components::{ButtonReturn, ButtonWindowMode, MouseInverted, TextWindowMode},
    constants::{COLOR_HOVERED, COLOR_SELECTED, COLOR_UNSELECTED},
    functions::window_mode_text,
};
use crate::{
    assets::LocaleAsset,
    components::{Language, RootComponent},
    functions::{rescale_board, switch_state},
    resources::{LoadState, LocaleAssets, Profile, TextSettings},
    states::AppState,
};
use bevy::{
    input::Input,
    prelude::{
        Assets, EventReader, KeyCode, ParamSet, Query, Res, ResMut, Sprite, State, Transform, With,
        Without,
    },
    text::Text,
    window::{WindowResized, Windows},
};
use interactable::components::{Entered, Exited, ReleasedLeft};

pub fn lang_click_system(
    lang_query: Query<&Language, With<ReleasedLeft>>,
    (mut app_state, mut load_state, mut profile): (
        ResMut<State<AppState>>,
        ResMut<LoadState>,
        ResMut<Profile>,
    ),
) {
    for lang in lang_query.iter() {
        if profile.lang != *lang {
            profile.lang = *lang;
            switch_state(Some(AppState::Options), &mut app_state, &mut load_state);
        }
    }
}

pub fn mouse_setting_click_system(
    mut mi_query: Query<(&MouseInverted, &mut Sprite), With<ReleasedLeft>>,
    mut other_query: Query<&mut Sprite, (With<MouseInverted>, Without<ReleasedLeft>)>,
    mut profile: ResMut<Profile>,
) {
    for (mi, mut sprite) in mi_query.iter_mut() {
        profile.mouse_inverted = mi.0;
        sprite.color = COLOR_SELECTED;
        for mut sprite in other_query.iter_mut() {
            sprite.color = COLOR_UNSELECTED;
        }
    }
}

pub fn lang_hover_system(
    mut hover_set: ParamSet<(
        Query<(&mut Sprite, &Language), With<Entered>>,
        Query<(&mut Sprite, &Language), With<Exited>>,
    )>,
    profile: Res<Profile>,
) {
    for (mut sprite, lang) in hover_set.p0().iter_mut() {
        if profile.lang != *lang {
            sprite.color = COLOR_HOVERED;
        }
    }
    for (mut sprite, lang) in hover_set.p1().iter_mut() {
        if profile.lang != *lang {
            sprite.color = COLOR_UNSELECTED;
        }
    }
}

pub fn mouse_setting_hover_system(
    mut hover_set: ParamSet<(
        Query<(&mut Sprite, &MouseInverted), With<Entered>>,
        Query<(&mut Sprite, &MouseInverted), With<Exited>>,
    )>,
    profile: Res<Profile>,
) {
    for (mut sprite, mi) in hover_set.p0().iter_mut() {
        if profile.mouse_inverted != mi.0 {
            sprite.color = COLOR_HOVERED;
        }
    }
    for (mut sprite, mi) in hover_set.p1().iter_mut() {
        if profile.mouse_inverted != mi.0 {
            sprite.color = COLOR_UNSELECTED;
        }
    }
}

pub fn window_mode_button_click_system(
    button_query: Query<&ButtonWindowMode, With<ReleasedLeft>>,
    mut text_query: Query<&mut Text, With<TextWindowMode>>,
    (locale, mut profile, text_settings): (Res<LocaleAssets>, ResMut<Profile>, Res<TextSettings>),
    locales: Res<Assets<LocaleAsset>>,
    mut wnds: ResMut<Windows>,
) {
    if !button_query.is_empty() {
        profile.fullscreen = !profile.fullscreen;
        for wnd in wnds.iter_mut() {
            if profile.fullscreen {
                wnd.set_mode(bevy::window::WindowMode::Fullscreen);
            } else {
                wnd.set_mode(bevy::window::WindowMode::Windowed);
                wnd.set_maximized(true);
            }
        }
        if let Ok(mut text) = text_query.get_single_mut() {
            *text = window_mode_text(&locale, &locales, &profile, &text_settings);
        }
    }
}

pub fn return_button_click_system(
    return_query: Query<&ButtonReturn, With<ReleasedLeft>>,
    mut app_state: ResMut<State<AppState>>,
    mut load_state: ResMut<LoadState>,
) {
    if !return_query.is_empty() {
        switch_state(Some(AppState::Home), &mut app_state, &mut load_state);
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
            rescale_board(10, 6, 1, ev.width, ev.height, &mut root);
        }
    }
}
