use super::{
    components::{ButtonMenu, ButtonNext, ButtonRetry, UiBackground, UiRootNode},
    resources::{OverlaySettings, OverlayType},
};
use crate::{functions::switch_state, resources::LoadState, states::AppState};
use bevy::{
    hierarchy::DespawnRecursiveExt,
    input::Input,
    math::Vec3,
    prelude::{
        Commands, Entity, EventReader, KeyCode, Or, Query, Res, ResMut, State, Transform, With,
        Without,
    },
    window::WindowResized,
};
use interactable::click::{ClickType, MouseLeftClickEvent};

pub fn button_system(
    menu_button_query: Query<&ButtonMenu>,
    next_button_query: Query<&ButtonNext>,
    retry_button_query: Query<&ButtonRetry>,
    (mut app_state, mut load_state): (ResMut<State<AppState>>, ResMut<LoadState>),
    overlay_settings: Res<OverlaySettings>,
    mut ev_mouse_left_click: EventReader<MouseLeftClickEvent>,
) {
    for ev in ev_mouse_left_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok(_) = menu_button_query.get(ev.entity) {
            switch_state(
                Some(AppState::LevelSelection),
                &mut app_state,
                &mut load_state,
            );
        }
        if let Ok(_) = next_button_query.get(ev.entity) {
            assert!(overlay_settings.stage_id < 5);
            assert!(overlay_settings.level_id < 5);
            load_state.ids = Some((overlay_settings.stage_id, overlay_settings.level_id + 1));
            load_state.filename = Some(format!(
                "assets/levels/{}/{}.lvl",
                overlay_settings.stage_id + 1,
                overlay_settings.level_id + 2
            ));
            switch_state(Some(AppState::Level), &mut app_state, &mut load_state);
        }
        if let Ok(_) = retry_button_query.get(ev.entity) {
            switch_state(Some(AppState::Level), &mut app_state, &mut load_state);
        }
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

pub fn cleanup(
    mut commands: Commands,
    entities: Query<Entity, Or<(With<UiRootNode>, With<UiBackground>)>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
