use bevy::{
    hierarchy::DespawnRecursiveExt,
    input::Input,
    prelude::{Commands, Entity, KeyCode, Query, ResMut, State, With},
};

use crate::{functions::switch_state, resources::LoadState, states::AppState};

use super::components::UiRootNode;

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

pub fn cleanup(mut commands: Commands, entities: Query<Entity, With<UiRootNode>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
