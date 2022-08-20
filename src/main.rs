mod components;
mod constants;
mod functions;
mod helpers;
mod systems;

use bevy::{
    app::App,
    audio::AudioSource,
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{
        default, shape::RegularPolygon, AssetServer, Assets, Camera2dBundle, ClearColor, Color,
        Commands, Handle, Mesh, Msaa, ParallelSystemDescriptorCoercion, Res, ResMut, Transform,
    },
    sprite::{ColorMaterial, ColorMesh2dBundle},
    text::{TextAlignment, TextStyle},
    window::WindowDescriptor,
    DefaultPlugins,
};
use bevy_easings::EasingsPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
// use chrono::Utc;
use components::{
    Cell, CellColors, CellInner, CellOuter, EmptyCell, HiddenCell, NumberCell, SfxHover,
    TextSettings,
};
use constants::*;
use functions::spawn_cell_text;
use interactable::{
    click::Clickable, hover::Hoverable, shapes::*, InteractableCamera, InteractablePlugin,
};
use rand::{thread_rng, Rng};
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, 1.0))
                .with_translation(Vec3::new(0., 0., 999.9)),
            ..default()
        })
        .insert(InteractableCamera);
    // commands
    //     .spawn_bundle(Camera2dBundle::default())
    //     .insert(MainCamera);

    let mut rng = thread_rng();

    let medium_transform = Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX_CELL_OUTER));
    let small_transform = Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX_CELL_INNER));

    let big_hexagon = meshes.add(Mesh::from(RegularPolygon::new(RADIUS, 6)));
    let medium_hexagon = meshes.add(Mesh::from(RegularPolygon::new(RADIUS * 0.94, 6)));
    let small_hexagon = meshes.add(Mesh::from(RegularPolygon::new(RADIUS * 0.8, 6)));

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

    for x in 0..20 {
        for y in 0..20 {
            let tx = x as f32 * RADIUS * 1.56;
            let ty = (y + 1) as f32 * RADIUS * 1.8
                - match x % 2 {
                    0 => RADIUS * 0.9,
                    _ => 0.,
                };
            let rand_type = rng.gen_range(0..2);
            let rand_hidden = rng.gen_range(0..3);
            let colors = if rand_hidden == 0 {
                match rand_type {
                    0 => gray.clone(),
                    _ => blue.clone(),
                }
            } else {
                yellow.clone()
            };

            let mut big_transform =
                Transform::from_translation(Vec3::new(tx, ty, Z_INDEX_CELL_BACK));
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
                .id();

            commands.entity(cell).insert(Cell {
                x,
                y,
                entity: cell,
                outer_hexagon: child1,
                inner_hexagon: child2,
                orig: big_transform,
                hovering: false,
            });
            commands.entity(cell).push_children(&[child1, child2]);

            match rand_type {
                0 => {
                    let nc = NumberCell {
                        count: rng.gen_range(0..=6),
                    };
                    if rand_hidden == 0 {
                        spawn_cell_text(big_transform, &mut commands, &nc, &text_settings);
                    }
                    commands.entity(cell).insert(nc);
                }
                _ => {
                    commands.entity(cell).insert(EmptyCell);
                }
            }
            if rand_hidden > 0 {
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
                    clickable: Clickable {
                        ignore_scale: true,
                        shape: Shape::Hexagon(Hexagon {
                            radius: RADIUS,
                            point_up: false,
                        }),
                        left_released: true,
                        right_released: true,

                        ..default()
                    },
                });
            }
        }
    }
}
