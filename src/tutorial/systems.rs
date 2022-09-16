use super::components::{ButtonClose, ButtonNext, UiBackground, UiRootNode};
use crate::states::AppState;
use bevy::{
    input::Input,
    math::Vec3,
    prelude::{EventReader, KeyCode, Query, ResMut, State, Transform, With, Without},
    window::WindowResized,
};
use interactable::components::ReleasedLeft;

pub fn button_system(
    next_button_query: Query<&ButtonNext, With<ReleasedLeft>>,
    close_button_query: Query<&ButtonClose, With<ReleasedLeft>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if !next_button_query.is_empty() {}

    if !close_button_query.is_empty() {
        app_state.pop().unwrap();
    }
}

pub fn hotkey_system(mut app_state: ResMut<State<AppState>>, mut keys: ResMut<Input<KeyCode>>) {
    // Can only close overlay when paused.
    if keys.just_pressed(KeyCode::Escape) {
        keys.clear_just_pressed(KeyCode::Escape);
        app_state.pop().unwrap();
    }
}

/// On resizing the window, the ui is resized too
pub fn window_resize_system(
    mut ev_window_resize: EventReader<WindowResized>,
    mut root_query: Query<&mut Transform, With<UiRootNode>>,
    mut background_query: Query<&mut Transform, (With<UiBackground>, Without<UiRootNode>)>,
) {
    for ev in ev_window_resize.iter() {
        if let Ok(mut root) = root_query.get_single_mut() {
            let w = ev.width / 1920.;
            let h = ev.height / 1080.;
            let s = w.min(h);
            root.scale = Vec3::new(s, s, 1.0);
        }
        if let Ok(mut background) = background_query.get_single_mut() {
            background.scale = Vec3::new(ev.width, ev.height, 1.0);
        }
    }
}
