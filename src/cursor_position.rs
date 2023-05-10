use bevy::{prelude::*};

pub fn cursor_position(
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_q.single();

    if buttons.just_pressed(MouseButton::Left) {
        for window in windows.iter() {
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                eprintln!("World coords: {}/{}", world_position.x, world_position.y);
            }
        }
    }
}
