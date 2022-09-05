use bevy::{
    hierarchy::DespawnRecursiveExt,
    input::Input,
    math::Vec3,
    prelude::{
        Commands, Entity, EventReader, KeyCode, Or, Query, ResMut, State, Transform, With, Without,
    },
    window::WindowResized,
};

use crate::{functions::switch_state, resources::LoadState, states::AppState};

use super::components::{UiBackground, UiRootNode};

pub fn hotkey_system(
    mut app_state: ResMut<State<AppState>>,
    mut load_state: ResMut<LoadState>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
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
