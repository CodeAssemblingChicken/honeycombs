use bevy::{
    input::Input,
    prelude::{
        Camera, Commands, Entity, GlobalTransform, MouseButton, Query, Res, Transform, With,
    },
    utils::FloatOrd,
    window::Windows,
};

use crate::{common::mouse_to_world_pos, structs::Interactable, InteractableCamera};

pub fn interact_system(
    mut commands: Commands,
    mut interactable_query: Query<(Entity, &GlobalTransform, &mut Interactable)>,
    q_camera: Query<(&Camera, &Transform), With<InteractableCamera>>,
    wnds: Res<Windows>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    if let Some(pos) = mouse_to_world_pos(wnds, q_camera) {
        let mut interacted = Vec::new();
        let mut not_interacted = Vec::new();

        for (e, t, int) in interactable_query.iter() {
            if int.contains_point(pos, t) {
                interacted.push((e, FloatOrd(t.translation().z)));
            } else {
                not_interacted.push((e, FloatOrd(t.translation().z)));
            }
        }
        interacted.sort_by_key(|(_, z)| -*z);

        for (e, _) in &interacted {
            let (_, _, mut int) = interactable_query.get_mut(*e).unwrap();

            if !int.hovers.entered {
                int.hovers.entered = true;
            } else {
                int.hovers.entered = false;
            }
            int.hovers.hovered = true;
            int.hovers.exited = false;

            int.clicks.left_just = mouse_buttons.just_pressed(MouseButton::Left);
            int.clicks.left_pressed = mouse_buttons.pressed(MouseButton::Left);
            int.clicks.left_released = mouse_buttons.just_released(MouseButton::Left);

            int.clicks.right_just = mouse_buttons.just_pressed(MouseButton::Right);
            int.clicks.right_pressed = mouse_buttons.pressed(MouseButton::Right);
            int.clicks.right_released = mouse_buttons.just_released(MouseButton::Right);

            int.clicks.middle_just = mouse_buttons.just_pressed(MouseButton::Middle);
            int.clicks.middle_pressed = mouse_buttons.pressed(MouseButton::Middle);
            int.clicks.middle_released = mouse_buttons.just_released(MouseButton::Middle);

            if !int.pass_through {
                not_interacted.extend(interacted);
                break;
            }
        }
        for (e, _) in &not_interacted {
            let (_, _, mut int) = interactable_query.get_mut(*e).unwrap();
            let hovered = int.hovers.hovered;
            int.reset();
            if hovered {
                int.hovers.exited = true;
            }
        }
    }
}
