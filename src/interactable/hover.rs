use super::{
    common::{mouse_to_world_pos, Interactable},
    shapes::{ContainsPoint, Quad, Shape},
};
use crate::components::MainCamera;
use bevy::{
    math::Vec2,
    prelude::{
        Camera, Commands, Component, Entity, EventWriter, Query, Res, Transform, With, Without,
    },
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

impl Interactable for Hoverable {
    fn contains_point(&self, point: Vec2, tf: &Transform) -> bool {
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
