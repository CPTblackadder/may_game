use bevy::prelude::*;

use super::Velocity;

#[derive(Component)]
pub struct Charles1;

impl Plugin for Charles1 {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (raise_charles_1_arm, charles_1_movement).in_schedule(CoreSchedule::FixedUpdate),
        );
    }
}

#[derive(Component, Default)]
struct Charles1WobbleJoint {
    max: f32,
    min: f32,
    current_position: f32,
    acceleration: f32,
    velocity: f32,
    direction_positive: bool,
}

#[derive(Component)]
struct Charles1Arm;

pub fn create_charles_1(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let texture_handle_arm: Handle<Image> = asset_server.load("charles_1_arm.png");
    let texture_handle_torso: Handle<Image> = asset_server.load("charles_1_torso.png");
    let texture_handle_head: Handle<Image> = asset_server.load("charles_1_head.png");
    let texture_handle_bottom: Handle<Image> = asset_server.load("charles_1_bottom.png");

    let wobble_point_top = commands
        .spawn((
            Charles1WobbleJoint {
                max: 0.1,
                min: -0.1,
                direction_positive: true,
                acceleration: 0.01,
                ..Default::default()
            },
            SpatialBundle {
                transform: Transform::from_xyz(-18.0, -180.0, 0.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((SpriteBundle {
                    texture: texture_handle_torso.clone(),
                    transform: Transform::from_xyz(68.0, 123.0, 0.0),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Charles1WobbleJoint {
                                max: 0.05,
                                min: -0.05,
                                direction_positive: true,
                                acceleration: 0.003,
                                ..Default::default()
                            },
                            SpatialBundle {
                                transform: Transform::from_xyz(7.0, 58.0, 0.0),
                                ..Default::default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((SpriteBundle {
                                texture: texture_handle_head.clone(),
                                transform: Transform::from_xyz(-40.0, 250.0, 0.0),
                                ..Default::default()
                            },));
                        });
                });
            parent
                .spawn((
                    Charles1Arm,
                    SpatialBundle {
                        transform: Transform::from_xyz(-42.0, 265.0, 0.1),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((SpriteBundle {
                        texture: texture_handle_arm.clone(),
                        transform: Transform::from_xyz(-152.3, -177.0, 0.0),
                        ..Default::default()
                    },));
                });
        })
        .id();

    let wobble_point_bottom = commands
        .spawn((
            Charles1WobbleJoint {
                max: 0.2,
                min: -0.2,
                acceleration: 0.01,
                ..Default::default()
            },
            SpatialBundle {
                transform: Transform::from_xyz(-18.0, -180.0, -0.1),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((SpriteBundle {
                texture: texture_handle_bottom.clone(),
                transform: Transform::from_xyz(-44.1, -407.7, 0.0),
                ..Default::default()
            },));
        })
        .id();

    let mut charles_t = Transform::from_scale(Vec3 {
        x: 0.3,
        y: 0.3,
        z: 1.0,
    });
    charles_t.translation.z = 10.0;

    let charles_entity = commands
        .spawn((
            Charles1,
            Velocity::new(),
            SpatialBundle {
                transform: charles_t,
                ..Default::default()
            },
        ))
        .id();

    commands
        .entity(charles_entity)
        .push_children(&[wobble_point_top, wobble_point_bottom]);
}

fn raise_charles_1_arm(
    keys: Res<Input<KeyCode>>,
    mut arms: Query<&mut Transform, With<Charles1Arm>>,
    timer: Res<Time>,
) {
    for mut arm in arms.iter_mut() {
        let old_angle = arm.rotation.to_euler(EulerRot::XYZ).2;
        let new_angle;
        if keys.pressed(KeyCode::R) {
            new_angle = old_angle - 0.1;
        } else {
            new_angle = old_angle + 0.3;
        }

        arm.rotation = Quat::from_rotation_z(new_angle.clamp(-2.0, 0.0));
    }
}

fn charles_1_movement(
    velocities: Query<&Velocity, With<Charles1>>,
    mut wobblers: Query<(&mut Transform, &mut Charles1WobbleJoint)>,
) {
    if let Ok(charles_vel) = velocities.get_single() {
        for (mut w_trans, mut wobbler) in wobblers.iter_mut() {
            if charles_vel.value.x != 0.0 || charles_vel.value.y != 0.0 {
                // add a bit more wobblyiness
                wobbler.velocity += 0.01;
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

            if wobbler.velocity < 0.001 {
                // Reset towards 0
                wobbler.current_position *= 0.9;
            }

            w_trans.rotation = Quat::from_rotation_z(wobbler.current_position);
        }
    }
}
