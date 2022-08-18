use crate::{
    components::{Cell, CellColors, CellInner, CellOuter, HiddenCell},
    interactable::hover::{MouseEnterEvent, MouseExitEvent, MouseOverEvent},
};
use bevy::{
    hierarchy::Children,
    prelude::{
        ColorMaterial, Commands, EventReader, Handle, Query, ResMut, Transform, With, Without,
    },
};

pub fn mouse_click_cell(
    mut commands: Commands,
    cell_query: Query<(&Transform, &Cell, &Children)>,
    mut child_query_inner: Query<&mut Handle<ColorMaterial>, (With<CellInner>, Without<CellOuter>)>,
    mut child_query_outer: Query<&mut Handle<ColorMaterial>, (With<CellOuter>, Without<CellInner>)>,
    cell_colors: ResMut<CellColors>,
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok((t, cell, ch)) = cell_query.get(ev.0) {
            commands.entity(ev.0).remove_bundle::<HiddenCell>();
        }
    }
}

pub fn mouse_enter_cell(
    mut commands: Commands,
    cell_query: Query<(&Transform, &mut Cell)>,
    mut child_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: ResMut<CellColors>,
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok((t, cell)) = cell_query.get(ev.0) {
            cell.hover(&mut commands, t, &mut child_query, &cell_colors);
            // hover(&mut commands, ev.0, t);
        }
    }
}
pub fn mouse_exit_cell(
    mut commands: Commands,
    cell_query: Query<(&Transform, &Cell)>,
    mut child_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: ResMut<CellColors>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok((t, cell)) = cell_query.get(ev.0) {
            cell.unhover(&mut commands, t, &mut child_query, &cell_colors);
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
