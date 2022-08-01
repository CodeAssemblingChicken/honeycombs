mod components;
mod helpers;
mod systems;

use bevy::{
    app::App,
    core::Timer,
    math::Vec3,
    prelude::{
        AssetServer, Assets, Commands, Handle, Msaa, OrthographicCameraBundle, Res, Transform,
    },
    window::WindowDescriptor,
    DefaultPlugins,
};

use bevy_easings::EasingsPlugin;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use bevy_svg::prelude::{Origin, Svg, Svg2dBundle};
use components::{Cell, HiddenCell, MainCamera};
use rand::{thread_rng, Rng};
use systems::{click_cell, hover_system, wiggle, HoverEvent, WiggleTimer};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Hexacells".to_string(),
            ..Default::default()
        })
        .insert_resource(WiggleTimer(Timer::from_seconds(0.7, true)))
        .add_event::<HoverEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(helpers::camera::movement)
        .add_system(helpers::texture::set_texture_filters_to_nearest)
        // .add_system(click_cell)
        .add_system(hover_system)
        .add_system(wiggle)
        .register_inspectable::<Cell>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    let mut rng = thread_rng();

    let svgs: [Handle<Svg>; 3] = [
        asset_server.load("hex_black.svg"),
        asset_server.load("hex_blue.svg"),
        asset_server.load("hex_yellow.svg"),
    ];

    for x in 0..10 {
        for y in 0..10 {
            let tx = (x * 45) as f32;
            let ty = ((y + 1) * 50) as f32
                + match x % 2 {
                    0 => 0.,
                    _ => 25.,
                };
            commands
                .spawn_bundle(Svg2dBundle {
                    svg: svgs[rng.gen_range(0..3)].clone_weak(),
                    // origin: Origin::Center, // Origin::TopLeft is the default
                    ..Default::default()
                })
                .insert(Transform::from_translation(Vec3::new(tx, ty, 1.0)))
                .insert(Cell { x, y })
                .insert(HiddenCell);
        }
    }
}
