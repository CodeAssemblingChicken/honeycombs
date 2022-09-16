use super::{
    components::{ButtonReturn, MouseInverted},
    constants::{COLOR_HOVERED, COLOR_SELECTED, COLOR_UNSELECTED},
};
use crate::{
    components::{Language, RootComponent},
    functions::{rescale_board, switch_state},
    resources::{LoadState, Profile},
    states::AppState,
};
use bevy::{
    input::Input,
    prelude::{
        EventReader, KeyCode, ParamSet, Query, Res, ResMut, Sprite, State, Transform, With, Without,
    },
    window::WindowResized,
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
