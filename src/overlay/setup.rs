use super::components::UiRootNode;
use crate::{
    constants::Z_INDEX_UI,
    resources::{TextSettings, Viewport},
};
use bevy::{
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{default, shape::Quad, Assets, Color, Commands, Mesh, Res, ResMut, Transform},
    sprite::{ColorMaterial, ColorMesh2dBundle},
    text::{Text, Text2dBundle},
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    text_settings: Res<TextSettings>,
    viewport: Res<Viewport>,
) {
    let mut transform = Transform::from_translation(Vec3::new(0., 0., Z_INDEX_UI));
    let s = (5. * viewport.width / 1920.).max(5. * viewport.height / 1080.);
    transform.scale = Vec3::new(s, s, 1.);
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(Quad::new(Vec2::new(1920.0, 1080.0))))
                .into(),
            material: colors.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.3))),
            transform,
            ..default()
        })
        .insert(UiRootNode)
        .with_children(|parent| {
            parent
                .spawn_bundle(ColorMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(Quad::new(Vec2::new(640.0, 540.0))))
                        .into(),
                    material: colors.add(ColorMaterial::from(Color::rgba(0.8, 0.8, 0.8, 0.9))),
                    transform: Transform::from_translation(Vec3::new(0., 0., Z_INDEX_UI + 1.)),
                    ..default()
                })
                .insert(UiRootNode)
                .with_children(|parent| {
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::from_section("Pause", text_settings.style_menu.clone()),
                        transform: Transform::from_translation(Vec3::new(0., 0., Z_INDEX_UI + 2.)),
                        ..default()
                    });
                });
        });
}
