use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::{
    kills_required::PeasantKilled, peasant::Peasant, wobble_joint::WobbleJoint, CharlesVelocity,
    Shadow,
};

#[derive(Component)]
pub struct Charles1;

impl Plugin for Charles1 {
    fn build(&self, app: &mut App) {
        app.add_systems((raise_charles_1_arm,).in_schedule(CoreSchedule::FixedUpdate));
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Charles1Arm {
    state: Charles1ArmState,
}

#[derive(Default, PartialEq, Eq, Reflect)]
enum Charles1ArmState {
    #[default]
    AtRest,
    Rising,
    Falling,
}

pub fn create_charles_1(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // images from https://www.britannica.com/biography/Charles-I-king-of-Great-Britain-and-Ireland/images-videos#/media/1/106686/9308
    let texture_handle_arm: Handle<Image> = asset_server.load("charles_1_arm_2.png");
    let texture_handle_torso: Handle<Image> = asset_server.load("charles_1_torso.png");
    let texture_handle_head: Handle<Image> = asset_server.load("charles_1_head.png");
    let texture_handle_bottom: Handle<Image> = asset_server.load("charles_1_bottom.png");
    let shadow: Handle<Image> = asset_server.load("shadow.png");

    let mut charles_t = Transform::from_scale(Vec3 {
        x: 0.2,
        y: 0.2,
        z: 1.0,
    });
    charles_t.translation.z = 0.;

    let charles_entity = commands
        .spawn((
            Charles1,
            CharlesVelocity::new(true),
            SpatialBundle {
                transform: charles_t,
                ..Default::default()
            },
            crate::DeleteOnSceneChange,
            Name::new("Charles 1"),
        ))
        .id();

    let wobble_point_top = commands
        .spawn((
            WobbleJoint::new(charles_entity, 0.1, -0.1, 0.01, true),
            SpatialBundle {
                transform: Transform::from_xyz(-18.0, -180.0 + 950.0, 0.0),
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
                    Name::new("Body Sprite"),
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
                                Name::new("Head Sprite"),
                            ));
                        });
                });
            parent
                .spawn((
                    Charles1Arm::default(),
                    SpatialBundle {
                        transform: Transform::from_xyz(-42.0, 265.0, 0.0),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            texture: texture_handle_arm.clone(),
                            transform: Transform::from_xyz(-177.3, -463.5, 0.1),
                            ..Default::default()
                        },
                        Name::new("Arm Sprite"),
                    ));
                });
        })
        .id();

    let wobble_point_bottom = commands
        .spawn((
            WobbleJoint::new(charles_entity, 0.2, -0.2, 0.01, false),
            SpatialBundle {
                transform: Transform::from_xyz(-18.0, -180.0 + 950.0, 0.0),
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
                Name::new("Legs Sprite"),
            ));
        })
        .id();

    let shadow_entity = commands
        .spawn((
            SpriteBundle {
                texture: shadow.clone(),
                transform: Transform::from_xyz(-60., 0.0, 0.0),
                ..Default::default()
            },
            Shadow,
            Name::new("Shadow Sprite"),
        ))
        .id();

    let collider = commands
        .spawn((
            Name::new("Collider"),
            TransformBundle::default(),
            Collider::ball(180.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ))
        .id();

    commands.entity(charles_entity).push_children(&[
        wobble_point_top,
        wobble_point_bottom,
        shadow_entity,
        collider,
    ]);
}

fn raise_charles_1_arm(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut arms: Query<
        (&mut Transform, &mut Charles1Arm, &Children),
        (Without<Peasant>, With<Charles1Arm>),
    >,
    peasants: Query<(Entity, &Transform), (With<Peasant>, Without<Charles1Arm>)>,
    global_transforms: Query<&GlobalTransform>,
    mut lines: ResMut<DebugLines>,
    mut peasant_killed_event: EventWriter<PeasantKilled>,
) {
    for (mut transform, mut arm, child_entities) in arms.iter_mut() {
        if keys.pressed(KeyCode::Space) && arm.state == Charles1ArmState::AtRest {
            arm.state = Charles1ArmState::Rising;
        }

        let mut new_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        if arm.state == Charles1ArmState::Rising {
            new_angle -= 0.4;
        } else if arm.state == Charles1ArmState::Falling {
            new_angle += 0.25;
        }

        if new_angle >= 0.0 {
            new_angle = 0.0;
            arm.state = Charles1ArmState::AtRest;
        } else if new_angle <= -2.0 {
            new_angle = -2.0;
            arm.state = Charles1ArmState::Falling;
        }

        transform.rotation = Quat::from_rotation_z(new_angle);

        if arm.state == Charles1ArmState::Rising {
            for p in peasants.iter() {
                // Calculate bounding box
                let peasant_position = p.1.translation;
                let box_top_right: Vec2 = Vec2 {
                    x: peasant_position.x + 50.,
                    y: peasant_position.y + 200.,
                };
                let box_bottom_left: Vec2 = Vec2 {
                    x: peasant_position.x - 50.,
                    y: peasant_position.y,
                };

                lines.line(box_bottom_left.extend(0.), box_top_right.extend(0.), 0.5);

                // Check if any of the children of Charles1Arm are in it
                for e in child_entities.iter() {
                    let hit_point = global_transforms.get(*e).unwrap().translation();
                    let hit_point = Vec2::new(hit_point.x, hit_point.y);
                    if hit_point.x < box_top_right.x
                        && hit_point.y < box_top_right.y
                        && hit_point.x > box_bottom_left.x
                        && hit_point.y > box_bottom_left.y
                    {
                        peasant_killed_event.send(PeasantKilled(p.0));
                    }
                }
            }
        }
    }
}

pub fn check_peasant_takes_charles(
    mut collision_events: EventReader<CollisionEvent>,
    peasants: Query<Entity, (With<Peasant>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if let Ok(p) = peasants.get(*e1) {
                    println!("E1 was peasant");
                    next_state.set(AppState::Charles1);
                } else if let Ok(p) = peasants.get(*e2) {
                    println!("E2 was peasant");
                    next_state.set(AppState::Charles1);
                }
            }
            CollisionEvent::Stopped(_, _, _) => (),
        }
    }
}
