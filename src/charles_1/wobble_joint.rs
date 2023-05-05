use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use super::CharlesVelocity;

pub struct WobbleJointPlugin;

impl Plugin for WobbleJointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((wobbler_movement,).in_schedule(CoreSchedule::FixedUpdate));
    }
}

#[derive(Component)]
pub struct WobbleJoint {
    velocity_entity_id: Entity,
    max: f32,
    min: f32,
    current_position: f32,
    acceleration: f32,
    velocity: f32,
    centre: f32,
    direction_positive: bool,
}

impl WobbleJoint {
    pub fn new(
        velocity_entity_id: Entity,
        max: f32,
        min: f32,
        acceleration: f32,
        direction_positive: bool,
    ) -> WobbleJoint {
        assert_ne!(max, min);
        WobbleJoint {
            velocity_entity_id,
            max,
            min,
            acceleration,
            direction_positive,
            centre: (max + min) / 2.0,
            current_position: (max + min) / 2.0,
            velocity: 0.0,
        }
    }
}

fn wobbler_movement(
    charles_velocities: Query<&CharlesVelocity>,
    rapier_velocities: Query<&Velocity>,
    mut wobblers: Query<(&mut Transform, &mut WobbleJoint)>,
) {
    for (mut w_trans, mut wobbler) in wobblers.iter_mut() {
        let vel_vec;
        let charles = charles_velocities.get(wobbler.velocity_entity_id);
        let rapier = rapier_velocities.get(wobbler.velocity_entity_id);
        if let Ok(vel) = charles {
            vel_vec = vel.value;
        } else if let Ok(vel) = rapier {
            vel_vec = vel.linvel;
        } else {
            println!("Error, wobbler can't find velocity");
            return;
        }
        if vel_vec.x != 0.0 || vel_vec.y != 0.0 {
            // add a bit more wobblyiness
            wobbler.velocity += wobbler.acceleration;
        }
        wobbler.velocity *= 0.8;

        wobbler.current_position += (if wobbler.direction_positive {
            1.0
        } else {
            -1.0
        }) * wobbler.velocity;
        if wobbler.current_position > wobbler.max {
            wobbler.direction_positive = !wobbler.direction_positive;
            wobbler.current_position = wobbler.max;
        } else if wobbler.current_position < wobbler.min {
            wobbler.direction_positive = !wobbler.direction_positive;
            wobbler.current_position = wobbler.min;
        }

        if wobbler.velocity < 0.001 && wobbler.centre == 0.0 {
            // Reset towards centre
            wobbler.current_position *= 0.9;
        }

        w_trans.rotation = Quat::from_rotation_z(wobbler.current_position);
    }
}
