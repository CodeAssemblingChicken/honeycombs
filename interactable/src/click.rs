use crate::{
    common::{mouse_to_world_pos, Interactable},
    shapes::{ContainsPoint, Quad, Shape},
    InteractableCamera,
};
use bevy::{
    input::Input,
    math::Vec2,
    prelude::{
        Camera, Component, Entity, EventWriter, GlobalTransform, MouseButton, Query, Res,
        Transform, With,
    },
    window::Windows,
};

pub struct MouseLeftClickEvent {
    pub entity: Entity,
    pub click_type: ClickType,
}
pub struct MouseRightClickEvent {
    pub entity: Entity,
    pub click_type: ClickType,
}

pub struct MouseMiddleClickEvent {
    pub entity: Entity,
    pub click_type: ClickType,
}

#[derive(Default)]
pub struct MouseActions {
    pub left_just: bool,
    pub left_pressed: bool,
    pub left_released: bool,
    pub right_just: bool,
    pub right_pressed: bool,
    pub right_released: bool,
    pub middle_just: bool,
    pub middle_pressed: bool,
    pub middle_released: bool,
}

#[derive(Component)]
pub struct Clickable {
    pub ignore_scale: bool,
    pub pass_through: bool,
    pub shape: Shape,
    pub mouse_actions: MouseActions,
}

impl Interactable for Clickable {
    fn contains_point(&self, point: Vec2, tf: &GlobalTransform) -> bool {
        let scaling = match self.ignore_scale {
            true => None,
            false => Some(tf.affine().to_scale_rotation_translation().0.truncate()),
        };
        self.shape
            .contains_point(point, tf.translation().truncate(), scaling)
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
            mouse_actions: MouseActions::default(),
        }
    }
}

pub fn click_system(
    query: Query<(Entity, &GlobalTransform, &mut Clickable)>,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &Transform), With<InteractableCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    (mut left_click, mut right_click, mut middle_click): (
        EventWriter<MouseLeftClickEvent>,
        EventWriter<MouseRightClickEvent>,
        EventWriter<MouseMiddleClickEvent>,
    ),
) {
    if let Some(pos) = mouse_to_world_pos(wnds, q_camera) {
        let mut clicks = Vec::new();

        for (e, t, c) in query.iter() {
            if c.contains_point(pos, t) {
                clicks.push((e, c, t.translation().z));
            }
        }
        clicks.sort_by(|(_, _, z1), (_, _, z2)| z2.partial_cmp(z1).unwrap());

        for (e, c, _) in clicks {
            if c.mouse_actions.left_just && mouse_button_input.just_pressed(MouseButton::Left) {
                left_click.send(MouseLeftClickEvent {
                    entity: e,
                    click_type: ClickType::Just,
                })
            }
            if c.mouse_actions.left_pressed && mouse_button_input.pressed(MouseButton::Left) {
                left_click.send(MouseLeftClickEvent {
                    entity: e,
                    click_type: ClickType::Pressed,
                })
            }
            if c.mouse_actions.left_released && mouse_button_input.just_released(MouseButton::Left)
            {
                left_click.send(MouseLeftClickEvent {
                    entity: e,
                    click_type: ClickType::Released,
                })
            }
            if c.mouse_actions.right_just && mouse_button_input.just_pressed(MouseButton::Right) {
                right_click.send(MouseRightClickEvent {
                    entity: e,
                    click_type: ClickType::Just,
                })
            }
            if c.mouse_actions.right_pressed && mouse_button_input.pressed(MouseButton::Right) {
                right_click.send(MouseRightClickEvent {
                    entity: e,
                    click_type: ClickType::Pressed,
                })
            }
            if c.mouse_actions.right_released
                && mouse_button_input.just_released(MouseButton::Right)
            {
                right_click.send(MouseRightClickEvent {
                    entity: e,
                    click_type: ClickType::Released,
                })
            }
            if c.mouse_actions.middle_just && mouse_button_input.just_pressed(MouseButton::Middle) {
                middle_click.send(MouseMiddleClickEvent {
                    entity: e,
                    click_type: ClickType::Just,
                })
            }
            if c.mouse_actions.middle_pressed && mouse_button_input.pressed(MouseButton::Middle) {
                middle_click.send(MouseMiddleClickEvent {
                    entity: e,
                    click_type: ClickType::Pressed,
                })
            }
            if c.mouse_actions.middle_released
                && mouse_button_input.just_released(MouseButton::Middle)
            {
                middle_click.send(MouseMiddleClickEvent {
                    entity: e,
                    click_type: ClickType::Released,
                })
            }

            if !c.pass_through {
                break;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ClickType {
    Just,
    Pressed,
    Released,
}
