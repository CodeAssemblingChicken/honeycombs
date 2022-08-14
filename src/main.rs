mod components;
mod helpers;
mod interactable;
mod systems;

use bevy::{
    app::App,
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{
        default, shape::RegularPolygon, AssetServer, Assets, Camera2dBundle, Color, Commands, Mesh,
        Msaa, Res, ResMut, Transform,
    },
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::WindowDescriptor,
    DefaultPlugins,
};

use bevy_easings::EasingsPlugin;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use chrono::Utc;
use components::{Cell, HiddenCell, HoveredCell, MainCamera};
use interactable::{hover::Hoverable, shapes::*};
use rand::{thread_rng, Rng};
use systems::{click_cell, hover_system, wiggle};

pub const RADIUS: f32 = 25.0;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Hexacells".to_string(),
            ..Default::default()
        })
        .insert_resource(HoveredCell {
            coords: None,
            entity: None,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        .add_plugin(Material2dPlugin::<CustomMaterial>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(helpers::camera::movement)
        .add_system(helpers::texture::set_texture_filters_to_nearest)
        // .add_system(click_cell)
        // .add_system(hover_system)
        .add_system(interactable::hover::hover_system)
        .add_system(wiggle)
        .register_inspectable::<Cell>()
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    let mut rng = thread_rng();

    let big_hexagon = RegularPolygon::new(RADIUS, 6);
    let medium_hexagon = RegularPolygon::new(RADIUS * 0.94, 6);
    let small_hexagon = RegularPolygon::new(RADIUS * 0.76, 6);

    let yellow = (Color::hex("d4aa00").unwrap(), Color::hex("ffcc00").unwrap());
    let blue = (Color::hex("0088aa").unwrap(), Color::hex("00aad4").unwrap());
    let gray = (Color::hex("24221c").unwrap(), Color::hex("484537").unwrap());

    let medium_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 2.0));
    let small_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 3.0));

    for x in 0..4 {
        for y in 0..4 {
            let tx = (x * 45) as f32 + RADIUS;
            let ty = (y + 1) as f32 * RADIUS * 2.0
                - match x % 2 {
                    0 => RADIUS,
                    _ => 0.,
                };
            let rand = rng.gen_range(0..3);
            let colors = match rand {
                0 => yellow,
                1 => blue,
                _ => gray,
            };

            let mut big_transform = Transform::from_translation(Vec3::new(tx, ty, 1.0));
            big_transform.rotate_z(f32::to_radians(90.0));
            let cell = commands
                .spawn()
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: bevy::sprite::Mesh2dHandle(meshes.add(Mesh::from(big_hexagon))),
                    transform: big_transform,
                    material: materials.add(CustomMaterial {
                        color: Color::WHITE,
                    }),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(MaterialMesh2dBundle {
                        mesh: bevy::sprite::Mesh2dHandle(meshes.add(Mesh::from(medium_hexagon))),
                        transform: medium_transform,
                        material: materials.add(CustomMaterial { color: colors.0 }),
                        ..default()
                    });
                    parent.spawn_bundle(MaterialMesh2dBundle {
                        mesh: bevy::sprite::Mesh2dHandle(meshes.add(Mesh::from(small_hexagon))),
                        transform: small_transform,
                        material: materials.add(CustomMaterial { color: colors.1 }),
                        ..default()
                    });
                })
                .insert(Cell { x, y })
                .insert(Hoverable {
                    ignore_scale: true,
                    pass_through: false,
                    shape: Shape::Hexagon(Hexagon {
                        radius: RADIUS,
                        point_up: false,
                    }),
                    on_hover: Some(|_c, e, _t| println!("Yes {}", e.id())),
                    ..default() // width: 50.,
                                // height: 43.4,
                })
                .id();
            // .insert(HiddenCell);
            if rand == 0 {
                commands.entity(cell).insert(HiddenCell);
            }
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "d0165c41-d5c1-47cb-961c-335f1ab8b274"]
struct CustomMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}
