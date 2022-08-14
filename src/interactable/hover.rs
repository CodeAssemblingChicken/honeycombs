use bevy::{
    math::Vec2,
    prelude::{Camera, Commands, Component, Entity, Query, Res, Transform, With, Without},
    render::camera::RenderTarget,
    window::Windows,
};

use crate::components::MainCamera;

use super::shapes::{ContainsPoint, Quad, Shape};

#[derive(Debug, Component)]
pub struct Hovering;
#[derive(Debug, Component)]
pub struct JustHovered;
#[derive(Debug, Component)]
pub struct StopHovering;

#[derive(Component)]
pub struct Hoverable {
    pub ignore_scale: bool,
    pub pass_through: bool,
    pub shape: Shape,
    pub on_hover: Option<fn(&mut Commands, Entity, &Transform)>,
    pub on_enter: Option<fn(&mut Commands, Entity, &Transform)>,
    pub on_exit: Option<fn(&mut Commands, Entity, &Transform)>,
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
            on_hover: None,
            on_enter: None,
            on_exit: None,
        }
    }
}

pub fn hover_system(
    mut commands: Commands,
    mut hoverable_query: Query<(Entity, &Transform, &mut Hoverable)>,

    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &Transform), With<MainCamera>>,
) {
    if let Some(pos) = mouse_to_world_pos(wnds, q_camera) {
        let mut hovers = Vec::new();
        for (e, t, h) in hoverable_query.iter_mut() {
            if h.contains_point(pos, t) {
                hovers.push((e, h, t.translation.z));
            } else {
                commands.entity(e).insert(StopHovering);
            }
        }
        hovers.sort_by(|(_, _, z1), (_, _, z2)| z2.partial_cmp(z1).unwrap());

        for (e, h, _) in hovers {
            commands.entity(e).insert(JustHovered);
            if !h.pass_through {
                break;
            }
        }
    }
}

pub fn hovering(
    mut commands: Commands,
    mut hoverable_query: Query<
        (Entity, &Transform, &mut Hoverable),
        (With<Hovering>, Without<JustHovered>, Without<StopHovering>),
    >,
) {
    for (e, t, h) in hoverable_query.iter_mut() {
        h.on_hover.and_then(|f| Some(f(&mut commands, e, t)));
    }
}
pub fn just_hovered1(
    mut commands: Commands,
    mut hoverable_query: Query<
        (Entity, &Transform, &mut Hoverable),
        (With<JustHovered>, Without<Hovering>),
    >,
) {
    for (e, t, h) in hoverable_query.iter_mut() {
        h.on_enter.and_then(|f| Some(f(&mut commands, e, t)));
        commands.entity(e).remove::<JustHovered>();
        commands.entity(e).insert(Hovering);
    }
}
pub fn just_hovered2(
    mut commands: Commands,
    mut hoverable_query: Query<Entity, (With<Hoverable>, With<JustHovered>, With<Hovering>)>,
) {
    for e in hoverable_query.iter_mut() {
        commands.entity(e).remove::<JustHovered>();
    }
}
pub fn stop_hovering1(
    mut commands: Commands,
    mut hoverable_query: Query<
        (Entity, &Transform, &Hoverable),
        (With<StopHovering>, With<Hovering>),
    >,
) {
    for (e, t, h) in hoverable_query.iter_mut() {
        h.on_exit.and_then(|f| Some(f(&mut commands, e, t)));
        commands.entity(e).remove::<Hovering>();
    }
}
pub fn stop_hovering2(
    mut commands: Commands,
    mut hoverable_query: Query<Entity, (With<Hoverable>, With<StopHovering>)>,
) {
    for e in hoverable_query.iter_mut() {
        commands.entity(e).remove::<StopHovering>();
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
