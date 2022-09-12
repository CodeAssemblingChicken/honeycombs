use crate::{
    common::mouse_to_world_pos,
    components::{
        Entered, Exited, Hovered, Interactable, JustPressedLeft, JustPressedMiddle,
        JustPressedRight, PressedLeft, PressedMiddle, PressedRight, ReleasedLeft, ReleasedMiddle,
        ReleasedRight,
    },
    InteractableCamera,
};
use bevy::{
    input::Input,
    prelude::{
        Camera, Commands, Entity, GlobalTransform, MouseButton, Query, Res, Transform, With,
    },
    utils::FloatOrd,
    window::Windows,
};

type Hover<'a> = (Option<&'a Entered>, Option<&'a Hovered>, Option<&'a Exited>);

pub fn interact_system(
    mut commands: Commands,
    interactable_query: Query<(Entity, &GlobalTransform, &Interactable, Hover)>,
    q_camera: Query<(&Camera, &Transform), With<InteractableCamera>>,
    wnds: Res<Windows>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    if let Some(pos) = mouse_to_world_pos(wnds, q_camera) {
        let mut interacted = Vec::new();
        let mut not_interacted = Vec::new();

        for (e, t, int, _) in interactable_query.iter() {
            if interactable_query.get_component::<Exited>(e).is_ok() {
                commands.entity(e).remove::<Exited>();
            }
            if int.contains_point(pos, t) {
                interacted.push((e, int, FloatOrd(t.translation().z)));
            } else {
                not_interacted.push((e, int, FloatOrd(t.translation().z)));
            }
        }
        interacted.sort_by_key(|(_, _, z)| -*z);

        for (i, (e, int, _)) in interacted.clone().into_iter().enumerate() {
            if interactable_query.get_component::<Hovered>(e).is_err() {
                commands.entity(e).remove::<Exited>();
                commands.entity(e).insert(Entered).insert(Hovered);
            } else {
                commands.entity(e).remove::<Entered>();
            }

            if mouse_buttons.just_pressed(MouseButton::Left) {
                commands.entity(e).insert(JustPressedLeft);
            } else {
                commands.entity(e).remove::<JustPressedLeft>();
            }
            if mouse_buttons.pressed(MouseButton::Left) {
                commands.entity(e).insert(PressedLeft);
            } else {
                commands.entity(e).remove::<PressedLeft>();
            }
            if mouse_buttons.just_released(MouseButton::Left) {
                commands.entity(e).insert(ReleasedLeft);
            } else {
                commands.entity(e).remove::<ReleasedLeft>();
            }

            if mouse_buttons.just_pressed(MouseButton::Right) {
                commands.entity(e).insert(JustPressedRight);
            } else {
                commands.entity(e).remove::<JustPressedRight>();
            }
            if mouse_buttons.pressed(MouseButton::Right) {
                commands.entity(e).insert(PressedRight);
            } else {
                commands.entity(e).remove::<PressedRight>();
            }
            if mouse_buttons.just_released(MouseButton::Right) {
                commands.entity(e).insert(ReleasedRight);
            } else {
                commands.entity(e).remove::<ReleasedRight>();
            }

            if mouse_buttons.just_pressed(MouseButton::Middle) {
                commands.entity(e).insert(JustPressedMiddle);
            } else {
                commands.entity(e).remove::<JustPressedMiddle>();
            }
            if mouse_buttons.pressed(MouseButton::Middle) {
                commands.entity(e).insert(PressedMiddle);
            } else {
                commands.entity(e).remove::<PressedMiddle>();
            }
            if mouse_buttons.just_released(MouseButton::Middle) {
                commands.entity(e).insert(ReleasedMiddle);
            } else {
                commands.entity(e).remove::<ReleasedMiddle>();
            }

            if !int.pass_through {
                not_interacted.extend(interacted.into_iter().skip(i + 1));
                break;
            }
        }
        for (e, _, _) in &not_interacted {
            if interactable_query.get_component::<Hovered>(*e).is_ok() {
                commands.entity(*e).insert(Exited);
            }
            commands
                .entity(*e)
                .remove::<Entered>()
                .remove::<Hovered>()
                .remove::<JustPressedLeft>()
                .remove::<PressedLeft>()
                .remove::<ReleasedLeft>()
                .remove::<JustPressedRight>()
                .remove::<PressedRight>()
                .remove::<ReleasedRight>()
                .remove::<JustPressedMiddle>()
                .remove::<PressedMiddle>()
                .remove::<ReleasedMiddle>();
        }
    } else {
        for (e, _, _, _) in interactable_query.iter() {
            commands
                .entity(e)
                .remove::<Entered>()
                // .remove::<Hovered>()
                .remove::<JustPressedLeft>()
                .remove::<PressedLeft>()
                .remove::<ReleasedLeft>()
                .remove::<JustPressedRight>()
                .remove::<PressedRight>()
                .remove::<ReleasedRight>()
                .remove::<JustPressedMiddle>()
                .remove::<PressedMiddle>()
                .remove::<ReleasedMiddle>();
        }
    }
}
