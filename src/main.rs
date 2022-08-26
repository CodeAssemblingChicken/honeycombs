mod components;
mod constants;
mod end_screen;
mod helpers;
mod level;
mod main_menu;
mod resources;
mod states;

use bevy::{
    app::App,
    prelude::{Camera2dBundle, ClearColor, Color, Commands, Msaa, Res, ResMut, State, SystemSet},
    window::WindowDescriptor,
    DefaultPlugins,
};
use bevy_easings::EasingsPlugin;
use components::Cell;

use std::{io, panic};
// use chrono::Utc;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use interactable::{InteractableCamera, InteractablePlugin};
use level::resources::LevelFile;
use native_dialog::MessageDialog;
use resources::{CellColors, CellMeshes, SfxHover, TextSettings};
use states::AppState;
use std::io::Write;

fn main() {
    set_panic_hook();
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Hexacell".to_string(),
            ..Default::default()
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
    main_menu::prepare_main_menu(&mut app);

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
        app_state.set(AppState::MainMenu).unwrap();
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
