use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

#[derive(Reflect, Component, Debug, Default)]
#[reflect(Component)]
pub struct FallingSprite {
    pub floor: f32,
    pub rotation_speed: f32,
    pub velocity: Vec2,
}

const FALLING_SPRITE_GRAVITY: f32 = 0.019;

pub fn caluculate_falling_sprites(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut FallingSprite)>,
) {
    for (e, mut t, mut fs) in query.iter_mut() {
        fs.velocity.x *= 0.99999;
        fs.rotation_speed *= 0.99999;
        fs.velocity.y *= 0.99999;
        fs.velocity.y -= FALLING_SPRITE_GRAVITY;

        t.translation += fs.velocity.extend(0.0);

        // Calculate lowest point
        let rot = t.rotation.to_euler(EulerRot::XYZ).2 + fs.rotation_speed;
        if t.translation.y < fs.floor {
            t.translation.y = fs.floor;
            fs.velocity.y = -fs.velocity.y / 2.0;
            fs.velocity.x *= 0.8;
            fs.rotation_speed *= 0.8;
        }

        if fs.velocity.length_squared() < 0.0005 && fs.rotation_speed < 0.00005 {
            commands.entity(e).remove::<FallingSprite>();
        }

        t.rotation = Quat::from_rotation_z(rot);
    }
}
