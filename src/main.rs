mod board;
mod components;
mod constants;
mod functions;
mod helpers;
mod systems;

use bevy::{
    app::App,
    audio::AudioSource,
    prelude::{
        default, AssetServer, Assets, Camera2dBundle, ClearColor, Color, Commands, Handle, Mesh,
        Msaa, ParallelSystemDescriptorCoercion, Res, ResMut,
    },
    sprite::ColorMaterial,
    text::{TextAlignment, TextStyle},
    window::WindowDescriptor,
    DefaultPlugins,
};
use bevy_easings::EasingsPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use board::Board;
// use chrono::Utc;
use components::{Cell, CellColors, SfxHover, TextSettings};
use constants::*;

use helpers::parser::board_from_file;
use interactable::{InteractableCamera, InteractablePlugin};

use systems::{
    mouse_click_cell, mouse_enter_cell, mouse_exit_cell, mouse_over_cell, window_resize_system,
};

fn main() {
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
        .add_startup_system(setup)
        .add_system(helpers::camera::movement)
        .add_system(mouse_over_cell)
        .add_system(mouse_enter_cell.before(mouse_over_cell))
        .add_system(mouse_exit_cell.before(mouse_enter_cell))
        .add_system(mouse_click_cell)
        .add_system(window_resize_system);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Cell>();

    app.run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(InteractableCamera);

    let white = materials.add(ColorMaterial {
        color: Color::WHITE,
        ..default()
    });
    let yellow = (
        materials.add(ColorMaterial {
            color: Color::hex("dc8c10").unwrap(),
            ..default()
        }),
        materials.add(ColorMaterial {
            color: Color::hex("e4a020").unwrap(),
            ..default()
        }),
    );
    let gray = (
        materials.add(ColorMaterial {
            color: Color::hex("24221c").unwrap(),
            ..default()
        }),
        materials.add(ColorMaterial {
            color: Color::hex("484537").unwrap(),
            ..default()
        }),
    );
    let blue = (
        materials.add(ColorMaterial {
            color: Color::hex("0088e8").unwrap(),
            ..default()
        }),
        materials.add(ColorMaterial {
            color: Color::hex("00a0f0").unwrap(),
            ..default()
        }),
    );

    commands.insert_resource(CellColors {
        white: white.clone(),
        yellow_dark: materials.add(ColorMaterial {
            color: Color::hex("d87408").unwrap(),
            ..default()
        }),
        yellow_medium: yellow.0.clone(),
        yellow_light: yellow.1.clone(),
        gray_dark: gray.0.clone(),
        gray_light: gray.1.clone(),
        blue_dark: blue.0.clone(),
        blue_light: blue.1.clone(),
    });

    let sfx_hover: Handle<AudioSource> = asset_server.load("sfx/hover.ogg");
    commands.insert_resource(SfxHover(sfx_hover));

    let font = asset_server.load("fonts/Purisa-Bold.otf");
    let text_style = TextStyle {
        font,
        font_size: (RADIUS * 1.3).round(),
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
