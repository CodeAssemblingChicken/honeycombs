use crate::functions::rescale_board;
use bevy::{
    prelude::{Camera, EventReader, Query, Transform, With},
    window::WindowResized,
};

/// On resizing the window, the board is resized too
/// i.e. the camera zoom (scale) is recalculated
pub fn window_resize_system(
    mut ev_window_resize: EventReader<WindowResized>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    // board: Res<Board>,
) {
    for ev in ev_window_resize.iter() {
        rescale_board(0, 0, 0, ev.width, ev.height, &mut camera_query);
    }
}
