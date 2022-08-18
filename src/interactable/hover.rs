use super::shapes::{ContainsPoint, Quad, Shape};
use crate::components::MainCamera;
use bevy::{
    math::Vec2,
    prelude::{
        Camera, Commands, Component, Entity, EventWriter, Query, Res, Transform, With, Without,
    },
    render::camera::RenderTarget,
    window::Windows,
};

pub struct MouseOverEvent(pub Entity);
pub struct MouseEnterEvent(pub Entity);
pub struct MouseExitEvent(pub Entity);

#[derive(Debug, Component)]
pub struct Hovering;

#[derive(Component)]
pub struct Hoverable {
    pub ignore_scale: bool,
    pub pass_through: bool,
    pub shape: Shape,
}

impl Hoverable {
    pub fn contains_point(&self, point: Vec2, tf: &Transform) -> bool {
        let scaling = match self.ignore_scale {
            true => None,
            false => Some(tf.scale.truncate()),
        };
        self.shape
            .contains_point(point, tf.translation.truncate(), scaling)
    }
}

impl Default for Hoverable {
    fn default() -> Self {
        Self {
            ignore_scale: false,
            pass_through: false,
            shape: Shape::Quad(Quad {
                width: 1.,
                height: 1.,
            }),
        }
    }
}

pub fn hover_system(
    mut commands: Commands,
    hovering_query: Query<(Entity, &Transform, &mut Hoverable), With<Hovering>>,
    not_hovering_query: Query<(Entity, &Transform, &mut Hoverable), Without<Hovering>>,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &Transform), With<MainCamera>>,
    mut ev_mouse_over: EventWriter<MouseOverEvent>,
    mut ev_mouse_enter: EventWriter<MouseEnterEvent>,
    mut ev_mouse_exit: EventWriter<MouseExitEvent>,
) {
    if let Some(pos) = mouse_to_world_pos(wnds, q_camera) {
        let mut hovers = Vec::new();

        for (e, t, h) in hovering_query.iter() {
            if h.contains_point(pos, t) {
                hovers.push((e, h, t.translation.z));
            } else {
                ev_mouse_exit.send(MouseExitEvent(e));
                commands.entity(e).remove::<Hovering>();
            }
        }
        for (e, t, h) in not_hovering_query.iter() {
            if h.contains_point(pos, t) {
                hovers.push((e, h, t.translation.z));
            }
        }
        hovers.sort_by(|(_, _, z1), (_, _, z2)| z2.partial_cmp(z1).unwrap());

        for (e, h, _) in hovers {
            if not_hovering_query.get(e).is_ok() {
                ev_mouse_enter.send(MouseEnterEvent(e));
                commands.entity(e).insert(Hovering);
            }
            ev_mouse_over.send(MouseOverEvent(e));
            if !h.pass_through {
                break;
            }
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
        if wnds.get(id).is_none() {
            return None;
        }
        wnds.get(id).unwrap()
    } else {
        if wnds.get_primary().is_none() {
            return None;
        }
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        return Some(world_pos);
    }
    return None;
}
