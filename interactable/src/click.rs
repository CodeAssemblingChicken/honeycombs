use crate::{
    common::{mouse_to_world_pos, Interactable},
    shapes::{ContainsPoint, Quad, Shape},
    InteractableCamera,
};
use bevy::{
    input::{mouse::MouseButtonInput, Input},
    math::Vec2,
    prelude::{
        Camera, Component, Entity, EventReader, EventWriter, MouseButton, Query, Res, Transform,
        With,
    },
    window::Windows,
};

pub struct MouseLeftJustEvent(pub Entity);
pub struct MouseLeftPressedEvent(pub Entity);
pub struct MouseLeftReleasedEvent(pub Entity);
pub struct MouseRightJustEvent(pub Entity);
pub struct MouseRightPressedEvent(pub Entity);
pub struct MouseRightReleasedEvent(pub Entity);

#[derive(Component)]
pub struct Clickable {
    pub ignore_scale: bool,
    pub pass_through: bool,
    pub shape: Shape,
    pub left_just: bool,
    pub left_pressed: bool,
    pub left_released: bool,
    pub right_just: bool,
    pub right_pressed: bool,
    pub right_released: bool,
}

impl Interactable for Clickable {
    fn contains_point(&self, point: Vec2, tf: &Transform) -> bool {
        let scaling = match self.ignore_scale {
            true => None,
            false => Some(tf.scale.truncate()),
        };
        self.shape
            .contains_point(point, tf.translation.truncate(), scaling)
    }
}

impl Default for Clickable {
    fn default() -> Self {
        Self {
            ignore_scale: false,
            pass_through: false,
            shape: Shape::Quad(Quad {
                width: 1.,
                height: 1.,
            }),
            left_just: false,
            left_pressed: false,
            left_released: false,
            right_just: false,
            right_pressed: false,
            right_released: false,
        }
    }
}

pub fn click_system(
    // mut commands: Commands,
    query: Query<(Entity, &Transform, &mut Clickable)>,
    // hovering_query: Query<(Entity, &Transform, &mut Hoverable), With<Hovering>>,
    // not_hovering_query: Query<(Entity, &Transform, &mut Hoverable), Without<Hovering>>,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &Transform), With<InteractableCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_button_input_events: EventReader<MouseButtonInput>,
    (mut lj, mut lp, mut lr, mut rj, mut rp, mut rr): (
        EventWriter<MouseLeftJustEvent>,
        EventWriter<MouseLeftPressedEvent>,
        EventWriter<MouseLeftReleasedEvent>,
        EventWriter<MouseRightJustEvent>,
        EventWriter<MouseRightPressedEvent>,
        EventWriter<MouseRightReleasedEvent>,
    ),
) {
    if mouse_button_input_events.is_empty() {
        return;
    }
    if let Some(pos) = mouse_to_world_pos(wnds, q_camera) {
        let mut clicks = Vec::new();

        for (e, t, c) in query.iter() {
            if c.contains_point(pos, t) {
                clicks.push((e, c, t.translation.z));
            }
        }
        clicks.sort_by(|(_, _, z1), (_, _, z2)| z2.partial_cmp(z1).unwrap());

        for (e, c, _) in clicks {
            if c.left_just && mouse_button_input.just_pressed(MouseButton::Left) {
                lj.send(MouseLeftJustEvent(e))
            }
            if c.left_pressed && mouse_button_input.pressed(MouseButton::Left) {
                lp.send(MouseLeftPressedEvent(e))
            }
            if c.left_released && mouse_button_input.just_released(MouseButton::Left) {
                lr.send(MouseLeftReleasedEvent(e))
            }
            if c.right_just && mouse_button_input.just_pressed(MouseButton::Right) {
                rj.send(MouseRightJustEvent(e))
            }
            if c.right_pressed && mouse_button_input.pressed(MouseButton::Right) {
                rp.send(MouseRightPressedEvent(e))
            }
            if c.right_released && mouse_button_input.just_released(MouseButton::Right) {
                rr.send(MouseRightReleasedEvent(e))
            }

            if !c.pass_through {
                break;
            }
        }
    }
}
