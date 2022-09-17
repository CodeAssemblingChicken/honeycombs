use crate::{
    assets::LocaleAsset,
    bundles::MenuButtonBundle,
    resources::{GameColors, LocaleAssets, Profile, TextSettings},
};

use super::{
    components::{ButtonReturn, UiRootNode},
    resources::DialogSettings,
};
use bevy::{
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{
        default, shape::Quad, Assets, Color, ColorMaterial, ColorMesh2dBundle, Commands, Mesh, Res,
        ResMut, Transform,
    },
    text::{Text, Text2dBundle},
    window::Windows,
};

type StandardResources<'a> = (
    Res<'a, DialogSettings>,
    Res<'a, GameColors>,
    Res<'a, LocaleAssets>,
    Res<'a, Profile>,
    Res<'a, TextSettings>,
);
type StandardAssets<'a> = (
    ResMut<'a, Assets<Mesh>>,
    ResMut<'a, Assets<ColorMaterial>>,
    Res<'a, Assets<LocaleAsset>>,
);
pub fn setup(
    mut commands: Commands,
    (dialog_settings, game_colors, locale, profile, text_settings): StandardResources,
    (mut meshes, mut colors, locales): StandardAssets,
    wnds: Res<Windows>,
) {
    let text = format!(
        "{}",
        locale
            .get_string(&dialog_settings.text, &locales, &profile)
            .unwrap_or(&"String not found".to_string())
    );
    let mut tf_panel = Transform::from_xyz(dialog_settings.x, dialog_settings.y, 0.9);
    for wnd in wnds.iter() {
        let w = wnd.width() / 1920.;
        let h = wnd.height() / 1080.;
        let s = w.min(h);
        tf_panel.scale = Vec3::new(s, s, 1.0);
    }
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(Quad::new(Vec2::new(
                    dialog_settings.width,
                    dialog_settings.height,
                ))))
                .into(),
            material: colors.add(ColorMaterial::from(Color::rgba(0.8, 0.8, 0.8, 0.9))),
            transform: tf_panel,
            ..default()
        })
        .insert(UiRootNode)
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(text, text_settings.style_menu_dark.clone())
                    .with_alignment(text_settings.alignment),
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            });
            parent
                .spawn_bundle(MenuButtonBundle::new(
                    Transform::from_xyz(0., 0., 0.9),
                    (240., 150.),
                    game_colors.menu_button.clone(),
                    &mut meshes,
                ))
                .with_children(|parent| {
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            locale
                                .get_string("return", &locales, &profile)
                                .unwrap_or(&"String not found".to_string()),
                            text_settings.style_menu_dark.clone(),
                        )
                        .with_alignment(text_settings.alignment),
                        transform: Transform::from_xyz(0., -10., 10.)
                            .with_scale(Vec3::new(0.75, 0.75, 1.)),
                        ..default()
                    });
                })
                .insert(ButtonReturn);
        });
}
