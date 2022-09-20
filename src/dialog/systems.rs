use super::components::{ButtonReturn, UiRootNode};
use crate::states::AppState;
use bevy::{
    input::Input,
    math::Vec3,
    prelude::{EventReader, KeyCode, Query, ResMut, State, Transform, With},
    window::WindowResized,
};
use interactable::components::ReleasedLeft;

pub fn button_system(
    return_button_query: Query<&ButtonReturn, With<ReleasedLeft>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if !return_button_query.is_empty() {
        app_state.pop().unwrap();
    }
}

pub fn hotkey_system(mut app_state: ResMut<State<AppState>>, mut keys: ResMut<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        keys.clear_just_pressed(KeyCode::Escape);
        app_state.pop().unwrap();
    }
}

/// On resizing the window, the ui is resized too
pub fn window_resize_system(
    mut ev_window_resize: EventReader<WindowResized>,
    mut root_query: Query<&mut Transform, With<UiRootNode>>,
) {
    for ev in ev_window_resize.iter() {
        if let Ok(mut root) = root_query.get_single_mut() {
            let w = ev.width / 1920.;
            let h = ev.height / 1080.;
            let s = w.min(h);
            root.scale = Vec3::new(s, s, 1.0);
        }
    }
}
