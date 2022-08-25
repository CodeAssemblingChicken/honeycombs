pub mod board;
pub mod components;
pub mod functions;
pub mod parser;
pub mod resources;
pub mod systems;

use self::{board::Board, systems::*};
use crate::{
    resources::{CellColors, TextSettings},
    states::AppState,
};
use bevy::{
    app::App,
    prelude::{Assets, Commands, Mesh, ParallelSystemDescriptorCoercion, ResMut, SystemSet},
};

const STATE: AppState = AppState::Level;

pub fn prepare_level(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(STATE).with_system(setup))
        .add_system_set(
            SystemSet::on_update(STATE)
                .with_system(mouse_over_cell)
                .with_system(mouse_enter_cell.before(mouse_over_cell))
                .with_system(mouse_exit_cell.before(mouse_enter_cell))
                .with_system(
                    mouse_click_cell
                        .after(mouse_enter_cell)
                        .after(mouse_exit_cell),
                )
                .with_system(window_resize_system),
        )
        .add_system_set(SystemSet::on_exit(STATE));
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    cell_colors: ResMut<CellColors>,
    text_settings: ResMut<TextSettings>,
) {
    // let white = materials.add(ColorMaterial::from(Color::WHITE));
    // let yellow = (
    //     materials.add(ColorMaterial::from(Color::hex("dc8c10").unwrap())),
    //     materials.add(ColorMaterial::from(Color::hex("e4a020").unwrap())),
    // );
    // let gray = (
    //     materials.add(ColorMaterial::from(Color::hex("37352a").unwrap())),
    //     materials.add(ColorMaterial::from(Color::hex("484537").unwrap())),
    // );
    // let blue = (
    //     materials.add(ColorMaterial::from(Color::hex("0088e8").unwrap())),
    //     materials.add(ColorMaterial::from(Color::hex("00a0f0").unwrap())),
    // );

    // commands.insert_resource(CellColors {
    //     white: white.clone(),
    //     yellow_dark: materials.add(ColorMaterial::from(Color::hex("d87408").unwrap())),
    //     yellow_medium: yellow.0.clone(),
    //     yellow_light: yellow.1.clone(),
    //     gray_dark: materials.add(ColorMaterial::from(Color::hex("24221c").unwrap())),
    //     gray_medium: gray.0.clone(),
    //     gray_light: gray.1.clone(),
    //     blue_dark: materials.add(ColorMaterial::from(Color::hex("0070e4").unwrap())),
    //     blue_medium: blue.0.clone(),
    //     blue_light: blue.1.clone(),
    // });

    let cells = parser::board_from_file("assets/levels/1/1.lvl");

    let b = Board::new(
        &mut commands,
        meshes,
        cells,
        &text_settings,
        cell_colors.white.clone(),
        (
            cell_colors.yellow_medium.clone(),
            cell_colors.yellow_light.clone(),
        ),
        (
            cell_colors.gray_medium.clone(),
            cell_colors.gray_light.clone(),
        ),
        (
            cell_colors.blue_medium.clone(),
            cell_colors.blue_light.clone(),
        ),
    );
    commands.spawn().insert(b);
}
