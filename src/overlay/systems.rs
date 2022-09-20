use super::{
    components::{ButtonMenu, ButtonRestart, ButtonVariable, UiBackground, UiRootNode},
    resources::{OverlaySettings, OverlayType},
};
use crate::{
    dialog::resources::DialogSettings,
    functions::switch_state,
    resources::{LoadState, Profile},
    states::AppState,
};
use bevy::{
    input::Input,
    math::Vec3,
    prelude::{EventReader, KeyCode, Query, Res, ResMut, State, Transform, With, Without},
    window::WindowResized,
};
use interactable::components::ReleasedLeft;

pub fn button_system(
    menu_button_query: Query<&ButtonMenu, With<ReleasedLeft>>,
    variable_button_query: Query<&ButtonVariable, With<ReleasedLeft>>,
    restart_button_query: Query<&ButtonRestart, With<ReleasedLeft>>,
    (mut app_state, mut dialog_settings, mut load_state): (
        ResMut<State<AppState>>,
        ResMut<DialogSettings>,
        ResMut<LoadState>,
    ),
    (overlay_settings, profile): (Res<OverlaySettings>, Res<Profile>),
) {
    if !menu_button_query.is_empty() {
        switch_state(
            Some(AppState::LevelSelection),
            &mut app_state,
            &mut load_state,
        );
    }
    for bt in variable_button_query.iter() {
        if bt.0 {
            if overlay_settings.level_id < 5 {
                load_state.ids = Some((overlay_settings.stage_id, overlay_settings.level_id + 1));
                load_state.filename = Some(format!(
                    "assets/levels/{}/{}.lvl",
                    overlay_settings.stage_id + 1,
                    overlay_settings.level_id + 2
                ));
                switch_state(Some(AppState::Level), &mut app_state, &mut load_state);
            } else if overlay_settings.level_id == 5 && overlay_settings.stage_id < 5 {
                if profile.is_unlocked(overlay_settings.stage_id + 1) {
                    load_state.ids = Some((overlay_settings.stage_id + 1, 0));
                    load_state.filename = Some(format!(
                        "assets/levels/{}/{}.lvl",
                        overlay_settings.stage_id + 2,
                        1
                    ));
                    switch_state(Some(AppState::Level), &mut app_state, &mut load_state);
                } else {
                    *dialog_settings = DialogSettings {
                        text: "need-points".to_string(),
                        width: 800.,
                        height: 400.,
                        x: 0.,
                        y: 0.,
                    };
                    app_state.push(AppState::Dialog).unwrap();
                }
            } else {
                *dialog_settings = DialogSettings {
                    text: "last-level".to_string(),
                    width: 800.,
                    height: 400.,
                    x: 0.,
                    y: 0.,
                };
                app_state.push(AppState::Dialog).unwrap();
            }
        } else {
            app_state.pop().unwrap();
        }
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
