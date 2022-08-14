use std::time::Duration;

use bevy::{
    math::Vec3,
    prelude::{Commands, Entity, Transform},
};
use bevy_easings::*;

pub fn hover(commands: &mut Commands, entity: Entity, t: &Transform) {
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
