use std::time::Duration;

use crate::components::{Cell, HiddenCell, MainCamera};
use bevy::{
    core::{Time, Timer},
    input::Input,
    math::{Vec2, Vec3},
    prelude::{
        Assets, Camera, Commands, Entity, EventReader, EventWriter, KeyCode, Query, Res, ResMut,
        Transform, With,
    },
    render::camera::RenderTarget,
    window::Windows,
};
use bevy_easings::*;
use bevy_svg::prelude::Svg;

pub fn click_cell(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &Transform), With<MainCamera>>,
    cell_query: Query<(Entity, &Cell, &Transform), With<HiddenCell>>,
) {
    if let Some(world_pos) = mouse_to_world_pos(wnds, q_camera) {
        println!("{:?}", world_pos);
        if let Some(cell) = world_pos_to_cell(world_pos, &cell_query) {
            println!("Yay");
        }
    }
}

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
    cell_query: &Query<(Entity, &Cell, &Transform), With<HiddenCell>>,
) -> Option<Entity> {
    for (entity, _cell, tf) in cell_query.iter() {
        if point_in_hexagon(world_pos, tf.translation.truncate()) {
            return Some(entity);
        }
    }
    return None;
}

const R: f32 = 25.0;
const R2: f32 = R * R;
const RI: f32 = 0.75 * R2;

fn point_in_hexagon(p: Vec2, center: Vec2) -> bool {
    let x = center + Vec2::new(25., -21.65);
    let p = p - x;

    let l2 = p.y * p.y + p.x * p.x;
    if l2 > R2 {
        return false;
    }
    // (sqrt(3)/2)^2 = 3/4
    if l2 < RI {
        return true;
    }

    return false;
}

fn point_in_hexagon_old(p: Vec2, center: Vec2) -> bool {
    // let x = center + Vec2::new(22.5, -25.);
    let p = (p - center) / 50.;
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

pub struct WiggleTimer(pub Timer);

pub fn wiggle(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<WiggleTimer>,
    mut event_hover_tile: EventReader<HoverEvent>,
    mut cell_query: Query<(Entity, &mut Transform), With<Cell>>,
    svgs: Res<Assets<Svg>>,
) {
    // if let Some(svg) = svgs.get("hex_black.svg") {
    //     println!("{}", svg.view_box);
    // }
    // return;
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for ev in event_hover_tile.iter() {
        // for (entity, mut t) in cell_query.iter_mut() {
        if let Ok((entity, t)) = cell_query.get(ev.0) {
            let mut t1 = t.clone();
            t1.scale = Vec3::new(1.1, 1.1, 1.);
            t1.translation += Vec3::new(-2.5, 2., 0.);
            commands.entity(entity).insert(
                t.ease_to(
                    t1,
                    EaseFunction::SineInOut,
                    EasingType::Once {
                        duration: Duration::from_millis(100),
                    },
                )
                .ease_to(
                    *t,
                    EaseFunction::SineInOut,
                    EasingType::Once {
                        duration: Duration::from_millis(100),
                    },
                ),
            );
        }
    }
}

pub struct HoverEvent(Entity);

pub struct HoveredTile {
    pub x: u32,
    pub y: u32,
}

pub fn hover_system(
    commands: Commands,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &Transform), With<MainCamera>>,
    // mut hovered_tile: ResMut<HoveredTile>,
    // selection: Res<Selection>,
    mut event_hover_tile: EventWriter<HoverEvent>,
    cell_query: Query<(Entity, &Cell, &Transform), With<HiddenCell>>,
) {
    if let Some(world_pos) = mouse_to_world_pos(windows, camera_query) {
        println!("{:?}", world_pos);
        if let Some(cell) = world_pos_to_cell(world_pos, &cell_query) {
            if let Ok((_, c, _t)) = cell_query.get(cell) {
                // println!("{:?}", _t.translation);
                event_hover_tile.send(HoverEvent(cell));
            }
        }
    }
}

pub fn hover_tile_system(
    mut commands: Commands,
    hovered_tile: Res<HoveredTile>,
    // selection: Res<Selection>,
    mut event_hover_update: EventReader<HoverEvent>,
) {
    for _ in event_hover_update.iter() {}
}
