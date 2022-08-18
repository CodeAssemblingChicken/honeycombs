use crate::{
    components::{Cell, CellColors, CellInner, CellOuter},
    interactable::hover::{MouseEnterEvent, MouseExitEvent, MouseOverEvent},
};
use bevy::{
    asset::HandleId,
    hierarchy::Children,
    math::Vec3,
    prelude::{
        ColorMaterial, Commands, Entity, EventReader, Handle, Query, ResMut, Transform, With,
        Without,
    },
};
use bevy_easings::*;
use std::time::Duration;

pub fn mouse_enter_cell(
    mut commands: Commands,
    cell_query: Query<(&Transform, &Children), With<Cell>>,
    mut child_query_inner: Query<&mut Handle<ColorMaterial>, (With<CellInner>, Without<CellOuter>)>,
    mut child_query_outer: Query<&mut Handle<ColorMaterial>, (With<CellOuter>, Without<CellInner>)>,
    cell_colors: ResMut<CellColors>,
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok((t, c)) = cell_query.get(ev.0) {
            hover(&mut commands, ev.0, t);
            for &child in c.iter() {
                if let Ok(mut h) = child_query_inner.get_mut(child) {
                    h.id = HandleId::from(&cell_colors.yellow_medium);
                }
                if let Ok(mut h) = child_query_outer.get_mut(child) {
                    h.id = HandleId::from(&cell_colors.yellow_dark);
                }
            }
        }
    }
}
pub fn mouse_exit_cell(
    mut commands: Commands,
    cell_query: Query<(&Transform, &Children), With<Cell>>,
    mut child_query_inner: Query<&mut Handle<ColorMaterial>, (With<CellInner>, Without<CellOuter>)>,
    mut child_query_outer: Query<&mut Handle<ColorMaterial>, (With<CellOuter>, Without<CellInner>)>,
    cell_colors: ResMut<CellColors>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok((t, c)) = cell_query.get(ev.0) {
            unhover(&mut commands, ev.0, t);
            for &child in c.iter() {
                if let Ok(mut h) = child_query_inner.get_mut(child) {
                    h.id = HandleId::from(&cell_colors.yellow_light);
                }
                if let Ok(mut h) = child_query_outer.get_mut(child) {
                    h.id = HandleId::from(&cell_colors.yellow_medium);
                }
            }
        }
    }
}
pub fn mouse_over_cell(
    mut commands: Commands,
    cell_query: Query<&Transform, With<Cell>>,
    mut ev_mouse_over: EventReader<MouseOverEvent>,
) {
    for ev in ev_mouse_over.iter() {
        // println!("{} is hovered.", ev.0.id());
    }
}

pub fn hover(commands: &mut Commands, entity: Entity, t: &Transform) {
    // println!("Enter");
    let mut t1 = t.clone();
    t1.scale = Vec3::new(1.06, 1.06, 1.);
    commands.entity(entity).insert(t.ease_to(
        t1,
        EaseFunction::ElasticOut,
        EasingType::Once {
            duration: Duration::from_millis(300),
        },
    ));
}
pub fn unhover(commands: &mut Commands, entity: Entity, t: &Transform) {
    // println!("Exit");
    let mut t1 = t.clone();
    t1.scale = Vec3::new(1.0, 1.0, 1.);
    commands.entity(entity).insert(t.ease_to(
        t1,
        EaseFunction::ElasticOut,
        EasingType::Once {
            duration: Duration::from_millis(300),
        },
    ));
}

// pub fn wiggle(
//     mut commands: Commands,
//     hovered_cell: Res<HoveredCell>,
//     cell_query: Query<(Entity, &mut Transform), With<Cell>>,
// ) {
//     if hovered_cell.is_changed() && hovered_cell.entity.is_some() {
//         if let Ok((entity, t)) = cell_query.get(hovered_cell.entity.unwrap()) {
//             let mut t0 = t.clone();
//             t0.scale = Vec3::new(1.0, 1.0, 1.);
//             let mut t1 = t.clone();
//             t1.scale = Vec3::new(1.03, 1.03, 1.);
//             let mut t2 = t.clone();
//             t2.scale = Vec3::new(0.98, 0.98, 1.);
//             commands.entity(entity).insert(
//                 t.ease_to(
//                     t1,
//                     EaseFunction::SineInOut,
//                     EasingType::Once {
//                         duration: Duration::from_millis(30),
//                     },
//                 )
//                 .ease_to(
//                     t2,
//                     EaseFunction::SineInOut,
//                     EasingType::Once {
//                         duration: Duration::from_millis(60),
//                     },
//                 )
//                 .ease_to(
//                     t1,
//                     EaseFunction::SineInOut,
//                     EasingType::Once {
//                         duration: Duration::from_millis(30),
//                     },
//                 ),
//             );
//         }
//     }
// }
