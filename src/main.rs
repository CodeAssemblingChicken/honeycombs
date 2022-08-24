mod components;
mod constants;
mod helpers;
mod level;
mod main_menu;
mod states;

use std::{io, panic};

use bevy::{
    app::App,
    audio::AudioSource,
    prelude::{
        AssetServer, Assets, Camera2dBundle, ClearColor, Color, Commands, Handle, Mesh, Msaa,
        ParallelSystemDescriptorCoercion, Res, ResMut, SystemSet,
    },
    sprite::ColorMaterial,
    text::{TextAlignment, TextStyle},
    window::WindowDescriptor,
    DefaultPlugins,
};
use bevy_easings::EasingsPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use components::Cell;
// use chrono::Utc;
use constants::*;

use interactable::{InteractableCamera, InteractablePlugin};

use level::{
    board::Board,
    parser::board_from_file,
    resources::{CellColors, SfxHover, TextSettings},
    systems::{
        mouse_click_cell, mouse_enter_cell, mouse_exit_cell, mouse_over_cell, window_resize_system,
    },
};
use native_dialog::MessageDialog;
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
        .add_plugins(DefaultPlugins)
        .add_plugin(InteractablePlugin)
        .add_plugin(EasingsPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state(AppState::Level)
        .add_system_set(SystemSet::on_enter(AppState::MainMenu))
        .add_system_set(SystemSet::on_update(AppState::MainMenu))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu))
        .add_system_set(SystemSet::on_enter(AppState::Level).with_system(setup_level))
        .add_system_set(
            SystemSet::on_update(AppState::Level)
                .with_system(mouse_over_cell)
                .with_system(mouse_enter_cell.before(mouse_over_cell))
                .with_system(mouse_exit_cell.before(mouse_enter_cell))
                .with_system(
                    mouse_click_cell
                        .after(mouse_enter_cell)
                        .after(mouse_exit_cell),
                )
                .with_system(window_resize_system),
        )
        .add_system_set(SystemSet::on_exit(AppState::Level))
        .add_startup_system(setup);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Cell>();

    app.run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(InteractableCamera);
}

fn setup_level(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let white = materials.add(ColorMaterial::from(Color::WHITE));
    let yellow = (
        materials.add(ColorMaterial::from(Color::hex("dc8c10").unwrap())),
        materials.add(ColorMaterial::from(Color::hex("e4a020").unwrap())),
    );
    let gray = (
        materials.add(ColorMaterial::from(Color::hex("24221c").unwrap())),
        materials.add(ColorMaterial::from(Color::hex("484537").unwrap())),
    );
    let blue = (
        materials.add(ColorMaterial::from(Color::hex("0088e8").unwrap())),
        materials.add(ColorMaterial::from(Color::hex("00a0f0").unwrap())),
    );

    commands.insert_resource(CellColors {
        white: white.clone(),
        yellow_dark: materials.add(ColorMaterial::from(Color::hex("d87408").unwrap())),
        yellow_medium: yellow.0.clone(),
        yellow_light: yellow.1.clone(),
        gray_dark: gray.0.clone(),
        gray_light: gray.1.clone(),
        blue_dark: blue.0.clone(),
        blue_light: blue.1.clone(),
    });

    let sfx_hover: Handle<AudioSource> = asset_server.load("sfx/hover.ogg");
    commands.insert_resource(SfxHover(sfx_hover));

    let font = asset_server.load("fonts/Harabara.ttf");
    let text_style = TextStyle {
        font,
        font_size: (RADIUS * 0.75).round(),
        color: Color::WHITE,
    };
    let text_settings = TextSettings {
        style: text_style,
        alignment: TextAlignment::CENTER,
    };
    commands.insert_resource(text_settings.clone());

    let cells = board_from_file("assets/levels/1/1.lvl");

    let b = Board::new(
        &mut commands,
        meshes,
        cells,
        &text_settings,
        white,
        yellow,
        gray,
        blue,
    );
    commands.spawn().insert(b);
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
