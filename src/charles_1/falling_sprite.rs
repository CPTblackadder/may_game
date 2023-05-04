use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct FallingSprite {
    pub floor: f32,
    pub rotation_speed: f32,
    pub velocity: Vec2,
    pub sprite_radious: f32,
}

const FALLING_SPRITE_GRAVITY: f32 = 0.01;

pub fn caluculate_falling_sprites(mut query: Query<(&mut Transform, &mut FallingSprite)>) {
    for (mut t, mut fs) in query.iter_mut() {
        fs.velocity.x *= 0.99999;
        fs.rotation_speed *= 0.99999;
        fs.velocity.y *= 0.99999;
        fs.velocity.y -= FALLING_SPRITE_GRAVITY;

        t.translation += fs.velocity.extend(0.0);

        // Calculate lowest point
        let rot = t.rotation.to_euler(EulerRot::XYZ).2 + fs.rotation_speed;
        if t.translation.y < fs.floor + fs.sprite_radious {
            t.translation.y = fs.floor;
            fs.velocity.y = -fs.velocity.y / 2.0;
            fs.velocity.x *= 0.8;
        }

        t.rotation = Quat::from_rotation_z(rot);
    }
}
