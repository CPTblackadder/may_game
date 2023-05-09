use bevy::{
    prelude::{shape::Circle, *},
    sprite::MaterialMesh2dBundle,
    utils::tracing::field::Visit,
};
use bevy_rapier2d::prelude::*;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{default, f32::consts::PI};

use crate::fading_sprite::FadingSprite;

use super::{
    character::Charles1, falling_sprite::FallingSprite, kills_required::PeasantKilled,
    wobble_joint::WobbleJoint, Shadow, PEASANT_MAX_HEALTH,
};

#[derive(Resource)]
pub struct FaceResources {
    pub happy_head: Handle<Image>,
    pub shocked_head: Handle<Image>,
}

#[derive(Component)]
pub struct Peasant {
    pub hit_by: [i32; PEASANT_MAX_HEALTH],
    pub head: Entity,
    pub crown: Entity,
    pub has_crown: bool,
}

pub fn reclaim_crown(
    mut killed_events: EventReader<PeasantKilled>,
    mut sprites: Query<&mut Visibility, With<Sprite>>,
    mut charles: Query<&mut Charles1>,
    mut peasants: Query<&mut Peasant>,
) {
    for k in killed_events.iter() {
        if let Ok(mut p) = peasants.get_mut(k.0) {
            if p.has_crown {
                let mut charles = charles.get_single_mut().unwrap();
                p.has_crown = false;
                let mut crown_sprite: Mut<Visibility> = sprites.get_mut(p.crown).unwrap();
                *crown_sprite = Visibility::Hidden;
                charles.has_crown = true;
                let mut crown_sprite = sprites.get_mut(charles.crown).unwrap();
                *crown_sprite = Visibility::Visible;
            }
        }
    }
}

pub fn destroy_peasant(
    mut commands: Commands,
    peasants: Query<(&Children, &Transform), With<Peasant>>,
    parent_child: Query<&Children>,
    mut sprites: Query<(Entity, &mut Handle<Image>), (With<Sprite>, Without<Shadow>)>,
    images: Res<Assets<Image>>,
    face_res: Res<FaceResources>,
    mut killed_events: EventReader<PeasantKilled>,
) {
    let mut rng: ThreadRng = rand::thread_rng();
    for k in killed_events.iter() {
        let p = k.0;
        if let Ok((children, t)) = peasants.get(p) {
            let floor = t.translation.y;
            apply_falling_sprite_rec(
                floor,
                &mut commands,
                children,
                &parent_child,
                &mut sprites,
                &images,
                &mut rng,
                &face_res,
            );
            commands.entity(p).despawn_recursive();
        }
    }
}

fn apply_falling_sprite_rec(
    floor: f32,
    mut commands: &mut Commands,
    children: &Children,
    parent_child: &Query<&Children>,
    mut sprites: &mut Query<(Entity, &mut Handle<Image>), (With<Sprite>, Without<Shadow>)>,
    images: &Res<Assets<Image>>,
    rng: &mut ThreadRng,
    face_res: &Res<FaceResources>,
) {
    for child in children {
        if let Ok(children) = parent_child.get(*child) {
            apply_falling_sprite_rec(
                floor,
                commands,
                children,
                parent_child,
                sprites,
                images,
                rng,
                face_res,
            );
        }
        if let Ok((entity, mut image_handle)) = sprites.get_mut(*child) {
            if *image_handle == face_res.happy_head {
                *image_handle = face_res.shocked_head.clone();
            }

            commands
                .entity(entity)
                .insert((
                    FallingSprite {
                        floor,
                        rotation_speed: rng.gen_range(-0.05..0.05),
                        velocity: Vec2 {
                            x: rng.gen_range(-1.3..1.4),
                            y: rng.gen_range(0.7..2.5),
                        },
                    },
                    crate::DeleteOnSceneChange,
                    FadingSprite::new(20.),
                ))
                .remove_parent_in_place()
                .despawn_descendants();
        }
    }
}

pub fn spawn_peasant(commands: &mut Commands, asset_server: &Res<AssetServer>, location: Vec2) {
    let texture_handle_legs: Handle<Image> = asset_server.load("peasant_legs.png");
    let texture_handle_body: Handle<Image> = asset_server.load("peasant_body.png");
    let texture_handle_head: Handle<Image> = asset_server.load("peasant_head_happy.png");
    let shadow: Handle<Image> = asset_server.load("shadow.png");
    // Preload shocked texture
    let _texture_handle_head: Handle<Image> = asset_server.load("peasant_head_shocked.png");
    let texture_handle_crown: Handle<Image> = asset_server.load("crown.png");

    let crown_entity = commands
        .spawn((
            SpriteBundle {
                texture: texture_handle_crown.clone(),
                transform: Transform::from_xyz(0., 250., 0.1).with_scale(Vec3::new(0.3, 0.3, 1.)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            Name::new("Crown Sprite"),
        ))
        .id();

    let mut peasant_transform: Transform = Transform::from_translation(location.extend(0.0));
    peasant_transform.scale = Vec3::new(0.2, 0.2, 1.0);
    let head_entity = commands
        .spawn((SpriteBundle {
            texture: texture_handle_head.clone(),
            transform: Transform::from_xyz(0.0, 200.0, 0.1),
            ..Default::default()
        },))
        .add_child(crown_entity)
        .id();
    let peasant_entity = commands
        .spawn((
            Peasant {
                hit_by: [-1; PEASANT_MAX_HEALTH],
                head: head_entity,
                crown: crown_entity,
                has_crown: false,
            },
            // Velocity::new(false),
            SpatialBundle {
                transform: peasant_transform,
                ..Default::default()
            },
            crate::DeleteOnSceneChange,
            RigidBody::Dynamic,
            GravityScale(0.0),
            Collider::ball(190.0),
            Velocity {
                linvel: Vec2 { x: 0.0, y: 0.0 },
                angvel: 0.0,
            },
            AdditionalMassProperties::Mass(0.1),
            ExternalForce {
                force: Vec2::new(0.0, 0.0),
                torque: 2.0,
            },
            ExternalImpulse {
                impulse: Vec2::new(0.0, 0.0),
                torque_impulse: 1.0,
            },
            Damping {
                linear_damping: 0.6,
                angular_damping: 1.0,
            },
            Friction::coefficient(0.0),
            Restitution {
                coefficient: 0.001,
                combine_rule: CoefficientCombineRule::Min,
            },
            Ccd::enabled(),
            LockedAxes::ROTATION_LOCKED, // Prevent rotating
        ))
        .id();

    let wobble_point_top = commands
        .spawn((
            WobbleJoint::new(peasant_entity, 0.1, -0.1, 0.008, true),
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 300.0, 0.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((SpriteBundle {
                    texture: texture_handle_body.clone(),
                    transform: Transform::from_xyz(0.0, 250.0, 0.2),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    parent
                        .spawn((
                            WobbleJoint::new(peasant_entity, 0.05, -0.05, 0.002, true),
                            SpatialBundle {
                                transform: Transform::from_xyz(0.0, 250.0, 0.0),
                                ..Default::default()
                            },
                        ))
                        .add_child(head_entity);
                });
            parent
                .spawn((
                    WobbleJoint::new(peasant_entity, PI + 0.2, PI - 0.2, 0.002, true),
                    SpatialBundle {
                        transform: Transform::from_xyz(0.0, 250.0, 0.0),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((SpriteBundle {
                        texture: texture_handle_legs.clone(),
                        transform: Transform::from_xyz(0.0, -150.0, 0.1),
                        ..Default::default()
                    },));
                });
        })
        .id();

    let wobble_point_bottom = commands
        .spawn((
            WobbleJoint::new(peasant_entity, 0.2, -0.2, 0.009, false),
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 300.0, 0.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((SpriteBundle {
                texture: texture_handle_legs.clone(),
                transform: Transform::from_xyz(0.0, -150.0, 0.2),
                ..Default::default()
            },));
        })
        .id();

    let shadow_entity = commands
        .spawn((
            SpriteBundle {
                texture: shadow.clone(),
                transform: Transform::from_xyz(0., 0.0, 0.0),
                ..Default::default()
            },
            Shadow,
            Name::new("Shadow Sprite"),
        ))
        .id();

    commands.entity(peasant_entity).push_children(&[
        wobble_point_top,
        wobble_point_bottom,
        shadow_entity,
    ]);
}

#[derive(Resource)]
pub struct PeasantTimer(pub Timer);

// Peasants stand around, when charles enters their cone of vision they give chase until charles is lost
pub fn periodically_spawn_peasants(
    mut commands: Commands,
    charles_1: Query<&Transform, With<Charles1>>,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<PeasantTimer>,
    peasants: Query<&Transform, With<Peasant>>,

    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let mut rand = thread_rng();

        let c_trans = charles_1.single();
        spawn_a_peasant(
            &mut commands,
            Vec2 {
                x: c_trans.translation.x,
                y: c_trans.translation.y,
            },
            &peasants,
            &asset_server,
            rand.gen_range(3..6),
        )
    }
}

fn spawn_a_peasant(
    mut commands: &mut Commands,
    charles_pos: Vec2,
    peasants: &Query<&Transform, With<Peasant>>,
    asset_server: &Res<AssetServer>,
    number: usize,
) {
    if peasants.iter().len() >= 30 {
        return;
    }
    fn points_too_close(a: Vec2, b: Vec2) -> bool {
        (a - b).length() < 300.
    }

    let mut rand = thread_rng();
    let attempts_made = 0;

    let mut peasnts_spawned = Vec::new();

    while peasnts_spawned.len() < number && attempts_made < 100 {
        let angle = rand.gen_range(0.0..2. * PI);
        let distance = rand.gen_range(600.0..1500.0);
        let spawn_point = (Vec2::from_angle(angle) * distance) + charles_pos;

        // Check spawn point
        if peasants
            .iter()
            .any(|x| points_too_close(Vec2::new(x.translation.x, x.translation.y), spawn_point))
            || peasnts_spawned
                .iter()
                .any(|x| points_too_close(*x, spawn_point))
        {
            continue;
        }
        peasnts_spawned.push(spawn_point);
        spawn_peasant(&mut commands, &asset_server, spawn_point);
    }

    for _ in 0..number {}
}

pub fn spawn_a_peasant_command(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    charles_1: Query<&Transform, With<Charles1>>,
    peasants: Query<&Transform, With<Peasant>>,
    asset_server: Res<AssetServer>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        let c_trans = charles_1.single();
        spawn_a_peasant(
            &mut commands,
            Vec2 {
                x: c_trans.translation.x,
                y: c_trans.translation.y,
            },
            &peasants,
            &asset_server,
            1,
        )
    }
}

pub fn add_velocity_towards_charles(
    mut peasants: Query<(&Transform, &mut ExternalForce, &Peasant)>,
    charles: Query<&Transform, With<Charles1>>,
) {
    let charles = charles.single();

    for (p_t, mut p_v, p) in peasants.iter_mut() {
        // Determine direction towards charles
        let direction = charles.translation - p_t.translation;
        let direction = Vec2::new(direction.x, direction.y);
        let modifier = if p.has_crown { -1. } else { 1. };

        // Set velocity towards charles
        p_v.force = direction.normalize() * 2. * modifier;
    }
}

pub fn cap_peasant_velocity(mut peasants: Query<&mut Velocity, With<Peasant>>) {
    for mut p in peasants.iter_mut() {
        p.linvel = p.linvel.clamp_length_max(3.0);
    }
}
