use crate::{
    components::{Language, RootComponent},
    functions::{rescale_board, switch_state},
    resources::{LoadState, Profile},
    states::AppState,
};
use bevy::{
    prelude::{
        Color, Entity, EventReader, Or, Query, Res, ResMut, Sprite, State, Transform, With, Without,
    },
    window::WindowResized,
};
use interactable::components::{Entered, Exited, ReleasedLeft};

use super::{
    components::MouseInverted,
    constants::{COLOR_HOVERED, COLOR_SELECTED, COLOR_UNSELECTED},
};

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
    mut enter_query: Query<(&mut Sprite, &Language), With<Entered>>,
    mut exit_query: Query<(&mut Sprite, &Language), (With<Exited>, Without<Entered>)>,
    profile: Res<Profile>,
) {
    for (mut sprite, lang) in enter_query.iter_mut() {
        if profile.lang != *lang {
            sprite.color = COLOR_HOVERED;
        }
    }
    for (mut sprite, lang) in exit_query.iter_mut() {
        if profile.lang != *lang {
            sprite.color = COLOR_UNSELECTED;
        }
    }
}

pub fn mouse_setting_hover_system(
    mut enter_query: Query<(&mut Sprite, &MouseInverted), With<Entered>>,
    mut exit_query: Query<(&mut Sprite, &MouseInverted), (With<Exited>, Without<Entered>)>,
    profile: Res<Profile>,
) {
    for (mut sprite, mi) in enter_query.iter_mut() {
        if profile.mouse_inverted != mi.0 {
            sprite.color = COLOR_HOVERED;
        }
    }
    for (mut sprite, mi) in exit_query.iter_mut() {
        if profile.mouse_inverted != mi.0 {
            sprite.color = COLOR_UNSELECTED;
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
