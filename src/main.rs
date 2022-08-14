mod components;
mod helpers;
mod interactable;
mod systems;

use bevy::prelude::ParallelSystemDescriptorCoercion;
use bevy::{
    app::App,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{
        default, shape::RegularPolygon, Assets, Camera2dBundle, ClearColor, Color, Commands, Mesh,
        Msaa, ResMut, Transform,
    },
    sprite::{ColorMaterial, ColorMesh2dBundle},
    window::WindowDescriptor,
    DefaultPlugins,
};

use bevy_easings::EasingsPlugin;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
// use chrono::Utc;
use components::{Cell, MainCamera};
use interactable::hover::{just_hovered2, stop_hovering1, stop_hovering2};
use interactable::{
    hover::{hover_system, hovering, just_hovered1, Hoverable},
    shapes::*,
};
use rand::{thread_rng, Rng};
use systems::{hover, unhover};
// use systems::wiggle;

pub const RADIUS: f32 = 25.0;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Hexacells".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.75, 0.75, 0.75)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(helpers::camera::movement)
        .add_system(helpers::texture::set_texture_filters_to_nearest)
        .add_system(hover_system)
        .add_system(hovering)
        .add_system(just_hovered1.before(hovering))
        .add_system(just_hovered2.after(just_hovered1))
        .add_system(stop_hovering1.before(hovering))
        .add_system(stop_hovering2.after(stop_hovering1))
        // .add_system(wiggle)
        .register_inspectable::<Cell>()
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    let mut rng = thread_rng();

    let medium_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 3.0));
    let small_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 4.0));

    let big_hexagon = meshes.add(Mesh::from(RegularPolygon::new(RADIUS, 6)));
    let medium_hexagon = meshes.add(Mesh::from(RegularPolygon::new(RADIUS * 0.94, 6)));
    let small_hexagon = meshes.add(Mesh::from(RegularPolygon::new(RADIUS * 0.76, 6)));

    let white = materials.add(ColorMaterial {
        color: Color::WHITE,
        ..default()
    });
    let yellow = (
        materials.add(ColorMaterial {
            color: Color::hex("d4aa00").unwrap(),
            ..default()
        }),
        materials.add(ColorMaterial {
            color: Color::hex("ffcc00").unwrap(),
            ..default()
        }),
    );
    let blue = (
        materials.add(ColorMaterial {
            color: Color::hex("0088aa").unwrap(),
            ..default()
        }),
        materials.add(ColorMaterial {
            color: Color::hex("00aad4").unwrap(),
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

    for x in 0..20 {
        for y in 0..20 {
            let tx = (x * 45) as f32 + RADIUS;
            let ty = (y + 1) as f32 * RADIUS * 2.0
                - match x % 2 {
                    0 => RADIUS,
                    _ => 0.,
                };
            let rand = rng.gen_range(0..3);
            let colors = match rand {
                0 => yellow.clone(),
                1 => blue.clone(),
                _ => gray.clone(),
            };

            let mut big_transform = Transform::from_translation(Vec3::new(tx, ty, 2.0));
            big_transform.rotate_z(f32::to_radians(90.0));
            let cell = commands
                .spawn()
                .insert_bundle(ColorMesh2dBundle {
                    mesh: big_hexagon.clone().into(),
                    material: white.clone().into(),
                    transform: big_transform,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(ColorMesh2dBundle {
                        mesh: medium_hexagon.clone().into(),
                        material: colors.0.into(),
                        transform: medium_transform,
                        ..default()
                    });
                    parent.spawn_bundle(ColorMesh2dBundle {
                        mesh: small_hexagon.clone().into(),
                        material: colors.1.into(),
                        transform: small_transform,
                        ..default()
                    });
                })
                .insert(Cell { x, y })
                .id();
            if rand == 0 {
                commands.entity(cell).insert(Hoverable {
                    ignore_scale: true,
                    pass_through: false,
                    shape: Shape::Hexagon(Hexagon {
                        radius: RADIUS,
                        point_up: false,
                    }),

                    // on_hover: Some(|mut c, e, t| println!("hov")),
                    // on_enter: Some(|mut c, e, t| println!("enter")),
                    // on_exit: Some(|mut c, e, t| println!("stop")),
                    on_enter: Some(|mut c, e, t| hover(&mut c, e, t)),
                    on_exit: Some(|mut c, e, t| unhover(&mut c, e, t)),
                    ..default()
                });
            }
        }
    }
}
