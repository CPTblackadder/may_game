use std::time::Duration;

use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::{
    kills_required::PeasantKilled,
    peasant::{FaceResources, Peasant},
    wobble_joint::WobbleJoint,
    CharlesVelocity, Shadow, PEASANT_MAX_HEALTH,
};

#[derive(Component)]
pub struct Charles1 {
    pub crown: Entity,
    pub has_crown: bool,
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
    let texture_handle_crown: Handle<Image> = asset_server.load("crown.png");
    let shadow: Handle<Image> = asset_server.load("shadow.png");

    let mut charles_t = Transform::from_scale(Vec3 {
        x: 0.2,
        y: 0.2,
        z: 1.0,
    });

    charles_t.translation.z = 0.;
    let crown_entity = commands
        .spawn((
            SpriteBundle {
                texture: texture_handle_crown.clone(),
                transform: Transform::from_xyz(-34., 170., 0.1).with_scale(Vec3::new(0.3, 0.3, 1.)),
                ..Default::default()
            },
            Name::new("Crown Sprite"),
        ))
        .id();

    let charles_entity = commands
        .spawn((
            Charles1 {
                crown: crown_entity,
                has_crown: true,
            },
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
                            parent
                                .spawn((
                                    SpriteBundle {
                                        texture: texture_handle_head.clone(),
                                        transform: Transform::from_xyz(-40.0, 250.0, 0.1),
                                        ..Default::default()
                                    },
                                    Name::new("Head Sprite"),
                                ))
                                .add_child(crown_entity);
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

pub fn raise_charles_1_arm(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut arms: Query<
        (&mut Transform, &mut Charles1Arm, &Children),
        (Without<Peasant>, Without<Charles1>),
    >,
    mut peasants: Query<
        (Entity, &Transform, &mut Peasant, &mut ExternalImpulse),
        (Without<Charles1Arm>),
    >,
    mut peasant_heads: Query<&mut Handle<Image>, (Without<Charles1Arm>)>,
    charles: Query<&Transform, With<Charles1>>,
    global_transforms: Query<&GlobalTransform>,
    mut peasant_killed_event: EventWriter<PeasantKilled>,
    mut times_raised: Local<i32>,
    faces: Res<FaceResources>,
    mut cooldown: Local<Timer>,
    time: Res<Time>,
) {
    let charles_pos;
    if let Ok(c) = charles.get_single() {
        charles_pos = c.translation;
    } else {
        return;
    }
    let charles_pos = Vec2::new(charles_pos.x, charles_pos.y);
    cooldown.tick(time.delta());
    for (mut transform, mut arm, child_entities) in arms.iter_mut() {
        if keys.pressed(KeyCode::Space)
            && arm.state == Charles1ArmState::AtRest
            && cooldown.finished()
        {
            arm.state = Charles1ArmState::Rising;
        }

        let mut new_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        if arm.state == Charles1ArmState::Rising {
            new_angle -= 0.4;
        } else if arm.state == Charles1ArmState::Falling {
            new_angle += 0.25;
        }

        if new_angle > 0.0 {
            new_angle = 0.0;
            arm.state = Charles1ArmState::AtRest;
            cooldown.set_duration(Duration::from_secs_f32(0.5));
            cooldown.reset();
            *times_raised += 1;
        } else if new_angle <= -2.0 {
            new_angle = -2.0;
            arm.state = Charles1ArmState::Falling;
        }

        transform.rotation = Quat::from_rotation_z(new_angle);

        if arm.state == Charles1ArmState::Rising {
            for mut p in peasants.iter_mut() {
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

                // Check if any of the children of Charles1Arm are in it
                for e in child_entities.iter() {
                    let hit_point = global_transforms.get(*e).unwrap().translation();
                    let hit_point = Vec2::new(hit_point.x, hit_point.y);
                    if hit_point.x < box_top_right.x
                        && hit_point.y < box_top_right.y
                        && hit_point.x > box_bottom_left.x
                        && hit_point.y > box_bottom_left.y
                    {
                        match calculate_peasant_damage(*times_raised, &mut p.2.hit_by) {
                            PeasantDamage::Hit => {
                                if let Ok(mut handle) = peasant_heads.get_mut(p.2.head) {
                                    *handle = faces.shocked_head.clone();
                                }

                                // peasant position - charles position
                                let force_dir = p.1.translation;
                                let force_dir =
                                    (Vec2::new(force_dir.x, force_dir.y) - charles_pos).normalize();
                                p.3.impulse = force_dir * 3.;
                            }
                            PeasantDamage::Killed => peasant_killed_event.send(PeasantKilled(p.0)),
                            PeasantDamage::None => (),
                        }
                    }
                }
            }
        }
    }
}

pub fn check_peasant_takes_charles(
    mut collision_events: EventReader<CollisionEvent>,
    mut peasants: Query<(&mut Peasant, &Transform, &mut ExternalImpulse)>,
    mut sprites: Query<&mut Visibility, With<Sprite>>,
    mut charles: Query<(&mut Charles1, &Transform)>,
) {
    let (mut charles, c_t) = charles.get_single_mut().unwrap();

    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if let Ok((mut p, p_t, mut f)) = peasants.get_mut(*e1) {
                    // Peasant steals crown
                    if charles.has_crown {
                        charles.has_crown = false;
                        let mut crown_sprite = sprites.get_mut(charles.crown).unwrap();
                        *crown_sprite = Visibility::Hidden;
                        p.has_crown = true;
                        let mut crown_sprite = sprites.get_mut(p.crown).unwrap();
                        *crown_sprite = Visibility::Visible;
                        // peasant position - charles position
                        let force_dir = p_t.translation - c_t.translation;
                        let force_dir = (Vec2::new(force_dir.x, force_dir.y)).normalize();
                        f.impulse = force_dir * 3.;
                    }
                } else if let Ok((mut p, p_t, mut f)) = peasants.get_mut(*e2) {
                    // Peasant steals crown
                    if charles.has_crown {
                        charles.has_crown = false;
                        let mut crown_sprite = sprites.get_mut(charles.crown).unwrap();
                        *crown_sprite = Visibility::Hidden;
                        p.has_crown = true;
                        let mut crown_sprite = sprites.get_mut(p.crown).unwrap();
                        *crown_sprite = Visibility::Visible;
                        // peasant position - charles position
                        let force_dir = p_t.translation - c_t.translation;
                        let force_dir = (Vec2::new(force_dir.x, force_dir.y)).normalize();
                        f.impulse = force_dir * 3.;
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => (),
        }
    }
}

enum PeasantDamage {
    Hit,
    Killed,
    None,
}

fn calculate_peasant_damage(hit_id: i32, hit_by: &mut [i32; PEASANT_MAX_HEALTH]) -> PeasantDamage {
    let mut i = 0;
    while i < PEASANT_MAX_HEALTH && !(hit_by[i] == -1 || hit_by[i] == hit_id) {
        i += 1;
    }
    if i == PEASANT_MAX_HEALTH {
        PeasantDamage::Killed
    } else if hit_by[i] == hit_id {
        PeasantDamage::None
    } else {
        hit_by[i] = hit_id;
        PeasantDamage::Hit
    }
}
