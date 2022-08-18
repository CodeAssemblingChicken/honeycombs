mod components;
mod helpers;
mod interactable;
mod systems;

use bevy::{
    app::App,
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{
        default, shape::RegularPolygon, Assets, Camera2dBundle, ClearColor, Color, Commands, Mesh,
        Msaa, ParallelSystemDescriptorCoercion, ResMut, Transform,
    },
    sprite::{ColorMaterial, ColorMesh2dBundle},
    window::WindowDescriptor,
    DefaultPlugins,
};
use bevy_easings::EasingsPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
// use chrono::Utc;
use components::{
    Cell, CellColors, CellInner, CellOuter, EmptyCell, HiddenCell, MainCamera, NumberCell,
};
use interactable::{
    hover::{hover_system, Hoverable, MouseEnterEvent, MouseExitEvent, MouseOverEvent},
    shapes::*,
};
use rand::{thread_rng, Rng};
use systems::{mouse_enter_cell, mouse_exit_cell, mouse_over_cell};
// use systems::wiggle;

pub const RADIUS: f32 = 25.0;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Hexacell".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.75, 0.75, 0.75)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_event::<MouseOverEvent>()
        .add_event::<MouseEnterEvent>()
        .add_event::<MouseExitEvent>()
        .add_startup_system(setup)
        .add_system(helpers::camera::movement)
        .add_system(helpers::texture::set_texture_filters_to_nearest)
        .add_system(mouse_over_cell)
        .add_system(mouse_enter_cell.before(mouse_over_cell))
        .add_system(mouse_exit_cell.before(mouse_over_cell))
        .add_system(hover_system);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Cell>();

    app.run();
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

            let b1 = ColorMesh2dBundle {
                mesh: medium_hexagon.clone().into(),
                material: colors.0.into(),
                transform: medium_transform,
                ..default()
            };
            let b2 = ColorMesh2dBundle {
                mesh: small_hexagon.clone().into(),
                material: colors.1.into(),
                transform: small_transform,
                ..default()
            };

            // do the same for the child
            let child1 = commands.spawn_bundle(b1).insert(CellOuter).id();
            let child2 = commands.spawn_bundle(b2).insert(CellInner).id();

            let cell = commands
                .spawn()
                .insert_bundle(ColorMesh2dBundle {
                    mesh: big_hexagon.clone().into(),
                    material: white.clone().into(),
                    transform: big_transform,
                    ..default()
                })
                // .with_children(|parent| {
                //     parent
                //         .spawn_bundle(ColorMesh2dBundle {
                //             mesh: medium_hexagon.clone().into(),
                //             material: c1,
                //             transform: medium_transform,
                //             ..default()
                //         })
                //         .insert(CellOuter);
                //     parent
                //         .spawn_bundle(ColorMesh2dBundle {
                //             mesh: small_hexagon.clone().into(),
                //             material: c2,
                //             transform: small_transform,
                //             ..default()
                //         })
                //         .insert(CellInner);
                // })
                .id();

            commands.entity(cell).insert(Cell {
                x,
                y,
                entity: Some(cell),
                outer_hexagon: Some(child1),
                inner_hexagon: Some(child2),
            });
            commands.entity(cell).push_children(&[child1, child2]);

            match rand {
                1 => {
                    commands.entity(cell).insert(NumberCell { count: 0 });
                }
                2 => {
                    commands.entity(cell).insert(EmptyCell);
                }
                _ => (),
            }
            if rand == 0 {
                commands.entity(cell).insert_bundle(HiddenCell {
                    hoverable: Hoverable {
                        ignore_scale: true,
                        pass_through: false,
                        shape: Shape::Hexagon(Hexagon {
                            radius: RADIUS,
                            point_up: false,
                        }),
                        ..default()
                    },
                });
            }
        }
    }

    commands.insert_resource(CellColors {
        white: white.clone(),
        yellow_dark: materials.add(ColorMaterial {
            color: Color::hex("aa8800").unwrap(),
            ..default()
        }),
        yellow_medium: yellow.0.clone(),
        yellow_light: yellow.1.clone(),
        gray_dark: gray.0.clone(),
        gray_light: gray.1.clone(),
        blue_dark: blue.0.clone(),
        blue_light: blue.1.clone(),
    });
}
