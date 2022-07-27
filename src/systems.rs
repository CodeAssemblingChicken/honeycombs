use std::time::Duration;

use crate::components::{Cell, HiddenCell, MainCamera};
use bevy::{
    input::Input,
    math::{Vec2, Vec3},
    prelude::{default, Camera, Commands, KeyCode, Query, Res, Transform, With},
    render::camera::RenderTarget,
    window::Windows,
};
use bevy_easings::{Ease, EaseFunction, EasingType};
use bevy_ecs_tilemap::{MapQuery, Tile, TilePos};
use rand::{thread_rng, Rng};

pub fn click_cell(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &Transform), With<MainCamera>>,
    cell_query: Query<(&Cell, &Transform), With<HiddenCell>>,
) {
    if let Some(world_pos) = mouse_to_world_pos(wnds, q_camera) {
        println!("{:?}", world_pos);
        if let Some(cell) = world_pos_to_cell(world_pos, cell_query) {
            println!("Yay");
        }
    }
}

fn hover_system() {}

fn mouse_to_world_pos(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &Transform), With<MainCamera>>,
) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        return Some(world_pos);
    }
    return None;
}

fn world_pos_to_cell(
    world_pos: Vec2,
    cell_query: Query<(&Cell, &Transform), With<HiddenCell>>,
) -> Option<Cell> {
    for (cell, tf) in cell_query.iter() {
        if point_in_hexagon(world_pos, tf.translation.truncate()) {
            return Some(*cell);
        }
    }
    return None;
}

fn point_in_hexagon(p: Vec2, center: Vec2) -> bool {
    // Check length (squared) against inner and outer radius
    // identity hexagon
    let l2 = p.y * p.y + p.x * p.x;
    if l2 > 1.0 {
        return false;
    }
    // (sqrt(3)/2)^2 = 3/4
    if l2 < 0.75 {
        return true;
    }

    // Check against borders
    let py = p.y * 1.15470053838; // 2/sqrt(3)
    if py > 1.0 || py < -1.0 {
        return false;
    }

    let px = 0.5 * py + p.x;
    if px > 1.0 || px < -1.0 {
        return false;
    }

    if py - px > 1.0 || py - px < -1.0 {
        return false;
    }

    return true;
}

pub fn wiggle(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut map_query: MapQuery,
    mut tile_query: Query<&mut Tile>,
) {
    if keyboard_input.just_pressed(KeyCode::V) {
        let mut random = thread_rng();
        let position = TilePos(1, 1);
        let tile_entity = map_query.get_tile_entity(position, 0u16, 0u16);

        if tile_entity.is_ok() {
            if let Ok(entity) = map_query.get_tile_entity(position, 0u16, 0u16) {
                if let Ok(mut tile) = tile_query.get_mut(entity) {
                    let t1 = Transform::from_translation(Vec3::new(0.0, 0.0, 1.0));
                    let t2 = Transform::from_translation(Vec3::new(100.0, 100.0, 1.0));
                    println!("Jo");
                    commands.entity(entity).insert(t1.ease_to(
                        t2,
                        EaseFunction::QuarticOut,
                        EasingType::Once {
                            duration: Duration::from_millis(1000),
                        },
                    ));
                    map_query.notify_chunk_for_tile(position, 0u16, 0u16);
                }
            }
        }
    }
}
