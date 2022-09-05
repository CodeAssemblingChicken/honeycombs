use super::components::{UiBackground, UiRootNode};
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
    window::Windows,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    text_settings: Res<TextSettings>,
    viewport: Res<Viewport>,
    wnds: Res<Windows>,
) {
    // Height and width 1920×1080p window
    let panel_width = 1280.;
    let panel_height = 720.;

    let mut tf_background = Transform::from_xyz(0., 0., Z_INDEX_UI);
    let mut tf_panel = Transform::from_xyz(0., 0., Z_INDEX_UI + 1.);
    for wnd in wnds.iter() {
        tf_background.scale = Vec3::new(wnd.width(), wnd.height(), 1.0);
        let w = wnd.width() / 1920.;
        let h = wnd.height() / 1080.;
        let s = w.min(h);
        tf_panel.scale = Vec3::new(s, s, 1.0);
    }
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(Quad::new(Vec2::new(1.0, 1.0))))
                .into(),
            material: colors.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.3))),
            transform: tf_background,
            ..default()
        })
        .insert(UiBackground);
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(Quad::new(Vec2::new(1280.0, 720.0))))
                .into(),
            material: colors.add(ColorMaterial::from(Color::rgba(0.8, 0.8, 0.8, 0.9))),
            transform: tf_panel,
            ..default()
        })
        .insert(UiRootNode)
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section("Pause", text_settings.style_menu.clone()),
                transform: Transform::from_xyz(0., 0., Z_INDEX_UI + 2.),
                ..default()
            });
        });
}
