#![windows_subsystem = "windows"]

mod board_functions;
mod components;
mod constants;
mod editor;
mod functions;
mod home;
mod level;
mod level_selection;
mod overlay;
mod parser;
mod resources;
mod states;

use bevy::{
    app::App,
    hierarchy::DespawnRecursiveExt,
    prelude::{
        default, Camera, Camera2dBundle, ClearColor, Color, Commands, Entity, Msaa, Query, Res,
        ResMut, State, SystemSet, Without,
    },
    window::{WindowDescriptor, WindowResizeConstraints},
    DefaultPlugins,
};
use bevy_asset_loader::prelude::AssetCollectionApp;
use bevy_easings::EasingsPlugin;
use bevy_kira_audio::AudioPlugin;
use overlay::resources::OverlaySettings;
use std::{
    io::{self, Write},
    panic,
};
// use chrono::Utc;
#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
#[cfg(feature = "bevy-inspector-egui")]
use components::Cell;
use interactable::{InteractableCamera, InteractablePlugin};
#[cfg(not(target_family = "wasm"))]
use native_dialog::MessageDialog;
use resources::{CellMeshes, GameColors, LoadState, Locale, Profile, SfxAssets, TextSettings};
use states::AppState;

fn main() {
    #[cfg(not(target_family = "wasm"))]
    set_panic_hook();
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Honeycombs".to_string(),
            resize_constraints: WindowResizeConstraints {
                min_width: 640.,
                min_height: 480.,
                max_width: f32::INFINITY,
                max_height: f32::INFINITY,
            },
            // mode: WindowMode::Fullscreen,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.15, 0.15, 0.15)))
        .insert_resource(LoadState::default())
        .insert_resource(OverlaySettings::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(InteractablePlugin)
        .add_plugin(EasingsPlugin)
        .add_plugin(AudioPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(save_profile_system)
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(load_complete));

    home::prepare_home(&mut app);
    level_selection::prepare_level_selection(&mut app);
    level::prepare_level(&mut app);
    editor::prepare_editor(&mut app);
    overlay::prepare_overlay(&mut app);

    #[cfg(feature = "bevy-inspector-egui")]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Cell>();

    let profile = Profile::new();

    app.init_resource::<CellMeshes>()
        .init_resource::<GameColors>()
        .init_resource::<TextSettings>()
        .insert_resource(Locale::new(&profile.lang))
        .insert_resource(profile)
        .init_collection::<SfxAssets>()
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(InteractableCamera);
}

fn save_profile_system(profile: Res<Profile>) {
    if profile.is_changed() {
        profile.save();
    }
}

fn load_complete(mut app_state: ResMut<State<AppState>>, load_state: Res<LoadState>) {
    app_state
        .set(load_state.next_state.unwrap_or_default())
        .unwrap();
}

pub fn cleanup(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn set_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let mut w = Vec::new();
        let _ = writeln!(&mut w, "{}", info);
        #[cfg(not(target_family = "wasm"))]
        MessageDialog::new()
            .set_type(native_dialog::MessageType::Error)
            .set_title("Error")
            .set_text(&format!(
                "Sorry, an error occurred. Please report it to the developer:\n{}",
                String::from_utf8(w).unwrap()
            ))
            .show_alert()
            .unwrap();
        let _ = writeln!(io::stderr(), "{}", info);
    }));
}
