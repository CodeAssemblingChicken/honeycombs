use super::{
    components::{ButtonMenu, ButtonNext, ButtonRestart, UiBackground, UiRootNode},
    resources::{OverlaySettings, OverlayType},
};
use crate::{functions::switch_state, resources::LoadState, states::AppState};
use bevy::{
    input::Input,
    math::Vec3,
    prelude::{EventReader, KeyCode, Query, Res, ResMut, State, Transform, With, Without},
    window::WindowResized,
};
use interactable::components::ReleasedLeft;

pub fn button_system(
    menu_button_query: Query<&ButtonMenu, With<ReleasedLeft>>,
    next_button_query: Query<&ButtonNext, With<ReleasedLeft>>,
    restart_button_query: Query<&ButtonRestart, With<ReleasedLeft>>,
    (mut app_state, mut load_state): (ResMut<State<AppState>>, ResMut<LoadState>),
    overlay_settings: Res<OverlaySettings>,
) {
    if !menu_button_query.is_empty() {
        switch_state(
            Some(AppState::LevelSelection),
            &mut app_state,
            &mut load_state,
        );
    }
    if !next_button_query.is_empty() {
        assert!(overlay_settings.level_id < 5);
        load_state.ids = Some((overlay_settings.stage_id, overlay_settings.level_id + 1));
        load_state.filename = Some(format!(
            "assets/levels/{}/{}.lvl",
            overlay_settings.stage_id + 1,
            overlay_settings.level_id + 2
        ));
        switch_state(Some(AppState::Level), &mut app_state, &mut load_state);
    }
    if !restart_button_query.is_empty() {
        switch_state(Some(AppState::Level), &mut app_state, &mut load_state);
    }
}

pub fn hotkey_system(
    mut app_state: ResMut<State<AppState>>,
    mut load_state: ResMut<LoadState>,
    mut keys: ResMut<Input<KeyCode>>,
    overlay_settings: Res<OverlaySettings>,
) {
    // Can only close overlay when paused.
    if keys.just_pressed(KeyCode::Escape) && overlay_settings.overlay_type == OverlayType::Pause {
        keys.clear_just_pressed(KeyCode::Escape);
        app_state.pop().unwrap();
    }
    if keys.just_pressed(KeyCode::M) {
        keys.clear_just_pressed(KeyCode::M);
        switch_state(
            Some(AppState::LevelSelection),
            &mut app_state,
            &mut load_state,
        );
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
