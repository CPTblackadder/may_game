use bevy::prelude::*;

use super::{wobble_joint::WobbleJoint, Velocity};

#[derive(Component)]
pub struct Charles1;

impl Plugin for Charles1 {
    fn build(&self, app: &mut App) {
        app.add_systems((raise_charles_1_arm,).in_schedule(CoreSchedule::FixedUpdate));
    }
}

#[derive(Component)]
struct Charles1Arm;

pub fn create_charles_1(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let texture_handle_arm: Handle<Image> = asset_server.load("charles_1_arm.png");
    let texture_handle_torso: Handle<Image> = asset_server.load("charles_1_torso.png");
    let texture_handle_head: Handle<Image> = asset_server.load("charles_1_head.png");
    let texture_handle_bottom: Handle<Image> = asset_server.load("charles_1_bottom.png");
    let shadow: Handle<Image> = asset_server.load("shadow.png");

    let mut charles_t = Transform::from_scale(Vec3 {
        x: 0.3,
        y: 0.3,
        z: 0.0,
    });
    charles_t.translation.z = 1.;

    let charles_entity = commands
        .spawn((
            Charles1,
            Velocity::new(true),
            SpatialBundle {
                transform: charles_t,
                ..Default::default()
            },
        ))
        .id();

    let wobble_point_top = commands
        .spawn((
            WobbleJoint::new(charles_entity, 0.1, -0.1, 0.01, true),
            SpatialBundle {
                transform: Transform::from_xyz(-18.0, -180.0, 0.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    SpriteBundle {
                        texture: texture_handle_torso.clone(),
                        transform: Transform::from_xyz(68.0, 123.0, 0.2),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            WobbleJoint::new(charles_entity, 0.05, -0.05, 0.003, true),
                            SpatialBundle {
                                transform: Transform::from_xyz(7.0, 58.0, 0.0),
                                ..Default::default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                SpriteBundle {
                                    texture: texture_handle_head.clone(),
                                    transform: Transform::from_xyz(-40.0, 250.0, 0.1),
                                    ..Default::default()
                                },
                            ));
                        });
                });
            parent
                .spawn((
                    Charles1Arm,
                    SpatialBundle {
                        transform: Transform::from_xyz(-42.0, 265.0, 0.0),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            texture: texture_handle_arm.clone(),
                            transform: Transform::from_xyz(-152.3, -177.0, 0.1),
                            ..Default::default()
                        },
                    ));
                });
        })
        .id();

    let wobble_point_bottom = commands
        .spawn((
            WobbleJoint::new(charles_entity, 0.2, -0.2, 0.01, false),
            SpatialBundle {
                transform: Transform::from_xyz(-18.0, -180.0, 0.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: texture_handle_bottom.clone(),
                    transform: Transform::from_xyz(-44.1, -407.7, 0.2),
                    ..Default::default()
                },
            ));
        })
        .id();

    let shadow_entity = commands
        .spawn((
            SpriteBundle {
                texture: shadow.clone(),
                transform: Transform::from_xyz(-60., -950.0, 0.0),
                ..Default::default()
            },
        ))
        .id();

    commands.entity(charles_entity).push_children(&[
        wobble_point_top,
        wobble_point_bottom,
        shadow_entity,
    ]);
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
