use bevy::{prelude::*, sprite::Sprite, utils::Instant};

#[derive(Component)]
pub struct FadingSprite {
    time_at_start_of_fade: Timer,
}

impl FadingSprite {
    pub fn new(time: f32) -> Self {
        FadingSprite {
            time_at_start_of_fade: Timer::from_seconds(time, TimerMode::Once),
        }
    }
}

pub fn fade_then_remove_fading_sprites(
    mut commands: Commands,
    mut sprites: Query<(Entity, &mut Sprite, &mut FadingSprite)>,
    time: Res<Time>,
) {
    for (e, mut sprite, mut fading) in sprites.iter_mut() {
        fading.time_at_start_of_fade.tick(time.delta());
        sprite
            .color
            .set_a(fading.time_at_start_of_fade.percent_left());

        if fading.time_at_start_of_fade.finished() {
            commands.entity(e).despawn_recursive();
        }
    }
}
