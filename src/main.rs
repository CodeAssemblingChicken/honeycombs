mod components;
mod constants;
mod helpers;
mod level;
mod main_menu;
mod resources;
mod states;

use std::{io, panic};

use bevy::{
    app::App,
    audio::AudioSource,
    prelude::{
        AssetServer, Assets, Camera2dBundle, ClearColor, Color, Commands, Handle, Msaa, Res,
        ResMut, State, SystemSet,
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

use native_dialog::MessageDialog;

use resources::{CellColors, SfxHover, TextSettings};
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
        .add_startup_system(setup)
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(show_menu_after_load));

    level::prepare_level(&mut app);
    main_menu::prepare_main_menu(&mut app);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Cell>();

    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(InteractableCamera);

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
    commands.insert_resource(CellColors {
        white: materials.add(ColorMaterial::from(Color::WHITE)),
        yellow_dark: materials.add(ColorMaterial::from(Color::hex("d87408").unwrap())),
        yellow_medium: materials.add(ColorMaterial::from(Color::hex("dc8c10").unwrap())),
        yellow_light: materials.add(ColorMaterial::from(Color::hex("e4a020").unwrap())),
        gray_dark: materials.add(ColorMaterial::from(Color::hex("24221c").unwrap())),
        gray_medium: materials.add(ColorMaterial::from(Color::hex("37352a").unwrap())),
        gray_light: materials.add(ColorMaterial::from(Color::hex("484537").unwrap())),
        blue_dark: materials.add(ColorMaterial::from(Color::hex("0070e4").unwrap())),
        blue_medium: materials.add(ColorMaterial::from(Color::hex("0088e8").unwrap())),
        blue_light: materials.add(ColorMaterial::from(Color::hex("00a0f0").unwrap())),
    });
}

fn show_menu_after_load(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::MainMenu).unwrap();
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
