use bevy::prelude::*;
use crate::bevy_tiling_background::BackgroundMovementScale;

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

// This function ensure things higher up the screen are further away
pub fn normalize_z_level(
    mut top_transforms: Query<
        &mut Transform,
        (
            Without<BackgroundMovementScale>,
            Without<Parent>,
            Without<Camera2d>,
            Changed<Transform>,
        ),
    >,
) {
    // Move z levels between 1.0 - 990.0 from y values of -50,000.0 - 50,000.0

    for mut t in top_transforms.iter_mut() {
        t.translation.z = (991.0 - map_range((-50000., 50000.), (1., 990.), t.translation.y));
    }
}
