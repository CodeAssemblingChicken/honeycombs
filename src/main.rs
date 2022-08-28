mod board_functions;
mod components;
mod constants;
mod editor;
mod end_screen;
mod functions;
mod level;
mod main_menu;
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
use bevy_easings::EasingsPlugin;
use components::Cell;
use std::{
    io::{self, Write},
    panic,
};
// use chrono::Utc;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use interactable::{InteractableCamera, InteractablePlugin};
use native_dialog::MessageDialog;
use resources::{CellColors, CellMeshes, LevelFile, SfxHover, TextSettings};
use states::AppState;

fn main() {
    set_panic_hook();
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Hexacell".to_string(),
            resize_constraints: WindowResizeConstraints {
                min_width: 640.,
                min_height: 480.,
                max_width: f32::INFINITY,
                max_height: f32::INFINITY,
            },
            // mode: WindowMode::Fullscreen,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.25, 0.25, 0.25)))
        .insert_resource(LevelFile::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(InteractablePlugin)
        .add_plugin(EasingsPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(show_menu_after_load));

    level::prepare_level(&mut app);
    editor::prepare_editor(&mut app);
    main_menu::prepare_main_menu(&mut app);
    end_screen::prepare_end_screen(&mut app);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Cell>();

    app.init_resource::<CellMeshes>()
        .init_resource::<CellColors>()
        .init_resource::<SfxHover>()
        .init_resource::<TextSettings>()
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(InteractableCamera);
}

fn show_menu_after_load(mut app_state: ResMut<State<AppState>>, level_file: Res<LevelFile>) {
    if level_file.filename.is_some() {
        app_state.set(AppState::Level).unwrap();
    } else {
        app_state.set(AppState::Editor).unwrap();
    }
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
        MessageDialog::new()
            .set_type(native_dialog::MessageType::Error)
            .set_title("Error")
            .set_text(&format!(
                "An error occurred, please report it to the developer:\n{}",
                String::from_utf8(w).unwrap()
            ))
            .show_alert()
            .unwrap();
        let _ = writeln!(io::stderr(), "{}", info);
    }));
}
