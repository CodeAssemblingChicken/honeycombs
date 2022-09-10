#![windows_subsystem = "windows"]

mod assets;
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

use assets::{LocaleAsset, LocaleAssetLoader};
use bevy::{
    app::{App, AppExit},
    hierarchy::DespawnRecursiveExt,
    prelude::{
        default, AddAsset, Camera, Camera2dBundle, ClearColor, Color, Commands, Entity,
        EventWriter, Msaa, Query, Res, ResMut, State, SystemSet, Without,
    },
    window::{WindowDescriptor, WindowResizeConstraints},
    DefaultPlugins,
};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_easings::EasingsPlugin;
#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use bevy_kira_audio::AudioPlugin;
#[cfg(feature = "bevy-inspector-egui")]
use components::Cell;
use interactable::{InteractableCamera, InteractablePlugin};
#[cfg(not(target_arch = "wasm32"))]
use native_dialog::MessageDialog;
use overlay::resources::OverlaySettings;
use resources::{
    CellMeshes, GameColors, LoadState, LocaleAssets, Profile, SfxAssets, TextSettings,
};
use states::AppState;
use std::{
    io::{self, Write},
    panic,
};

fn main() {
    // When building for native apps, use the native message dialog for panics
    #[cfg(not(target_arch = "wasm32"))]
    set_panic_hook();
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

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
        .init_resource::<LoadState>()
        .init_resource::<OverlaySettings>()
        .insert_resource(Profile::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(InteractablePlugin)
        .add_plugin(EasingsPlugin)
        .add_plugin(AudioPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(save_profile_system)
        // .add_state(AppState::Loading)
        .add_asset::<LocaleAsset>()
        .init_asset_loader::<LocaleAssetLoader>()
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::StateChange)
                .init_resource::<CellMeshes>()
                .init_resource::<GameColors>()
                .init_resource::<TextSettings>()
                // .init_resource::<Profile>()
                .with_collection::<SfxAssets>()
                .with_collection::<LocaleAssets>(),
        )
        .add_state(AppState::AssetLoading)
        .add_system_set(SystemSet::on_update(AppState::StateChange).with_system(load_complete))
        .add_system_set(SystemSet::on_enter(AppState::Settings).with_system(quit_system));

    home::prepare_home(&mut app);
    level_selection::prepare_level_selection(&mut app);
    level::prepare_level(&mut app);
    editor::prepare_editor(&mut app);
    overlay::prepare_overlay(&mut app);

    #[cfg(feature = "bevy-inspector-egui")]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Cell>();

    app.run();
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

fn quit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

fn set_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let mut w = Vec::new();
        let _ = writeln!(&mut w, "{}", info);
        #[cfg(not(target_arch = "wasm32"))]
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
