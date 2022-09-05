use super::{
    components::{UiBackground, UiRootNode},
    resources::OverlaySettings,
};
use crate::{
    constants::{RADIUS, Z_INDEX_CELL_BACK, Z_INDEX_UI},
    functions::spawn_cell,
    resources::{CellMeshes, GameColors, Locale, Profile, TextSettings},
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
    (mut meshes, mut colors): (ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>),
    (cell_meshes, game_colors, locale, overlay_settings, profile, text_settings): (
        Res<CellMeshes>,
        Res<GameColors>,
        Res<Locale>,
        Res<OverlaySettings>,
        Res<Profile>,
        Res<TextSettings>,
    ),
    wnds: Res<Windows>,
) {
    // Panel width and height 1920×1080p window
    let (panel_width, panel_height) = (1280., 720.);

    let mut point_cells = Vec::new();
    let points = profile.level_points[overlay_settings.stage_id as usize]
        [overlay_settings.level_id as usize]
        .unwrap_or_default();
    for i in 0..overlay_settings.max_points {
        let mut big_transform = Transform::from_xyz(
            (i / 5) as f32 * RADIUS / 1.2 - 560.,
            (i % 5) as f32 * RADIUS / -1.2 + 100.,
            Z_INDEX_CELL_BACK,
        )
        .with_scale(Vec3::new(0.4, 0.4, 1.0));
        big_transform.rotate_z(f32::to_radians(90.0));
        let colors = if i < points {
            (
                game_colors.alpha2.clone(),
                game_colors.blue_light.clone(),
                game_colors.blue_medium.clone(),
            )
        } else {
            (
                game_colors.alpha2.clone(),
                game_colors.gray_light.clone(),
                game_colors.gray_medium.clone(),
            )
        };
        let cell = commands.spawn().id();
        spawn_cell(
            &mut commands,
            cell,
            (
                cell_meshes.std_hexagon_back.clone(),
                cell_meshes.std_hexagon_outer.clone(),
                cell_meshes.std_hexagon_inner.clone(),
            ),
            colors,
            big_transform,
        );
        point_cells.push(cell);
    }

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
                .add(Mesh::from(Quad::new(Vec2::new(panel_width, panel_height))))
                .into(),
            material: colors.add(ColorMaterial::from(Color::rgba(0.8, 0.8, 0.8, 0.92))),
            transform: tf_panel,
            ..default()
        })
        .insert(UiRootNode)
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    locale
                        .get_string("pause")
                        .unwrap_or(&"String not found".to_string()),
                    text_settings.style_menu_dark.clone(),
                )
                .with_alignment(text_settings.alignment),
                transform: Transform::from_xyz(0., 250., Z_INDEX_UI + 2.),
                ..default()
            });
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section("Highscore:", text_settings.style_menu_dark.clone()),
                transform: Transform::from_xyz(-600., 50., Z_INDEX_UI + 2.),
                ..default()
            });
        })
        .push_children(&point_cells);
}
