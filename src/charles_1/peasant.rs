use std::{default, f32::consts::PI};

use bevy::{
    prelude::{shape::Circle, *},
    sprite::MaterialMesh2dBundle,
};

use crate::layer::ZLayer;

use super::{wobble_joint::WobbleJoint, Velocity};

#[derive(Component)]
struct Peasant;

#[derive(Default)]
enum PeasantState {
    #[default]
    Idle,
    Tracking,
}

pub fn spawn_peasant(commands: &mut Commands, asset_server: &Res<AssetServer>, location: Vec2) {
    let texture_handle_legs: Handle<Image> = asset_server.load("peasant_legs.png");
    let texture_handle_body: Handle<Image> = asset_server.load("peasant_body.png");
    let texture_handle_head: Handle<Image> = asset_server.load("peasant_head_happy.png");
    let shadow: Handle<Image> = asset_server.load("shadow.png");
    // Preload shocked texture
    let _texture_handle_head: Handle<Image> = asset_server.load("peasant_head_shocked.png");

    let mut peasant_transform: Transform = Transform::from_translation(location.extend(0.0));
    peasant_transform.scale = Vec3::new(0.3, 0.3, 1.0);

    let peasant_entity = commands
        .spawn((
            Peasant,
            Velocity::new(),
            SpatialBundle {
                transform: peasant_transform,
                ..Default::default()
            },
        ))
        .id();

    let wobble_point_top = commands
        .spawn((
            WobbleJoint::new(peasant_entity, 0.1, -0.1, 0.008, true),
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    SpriteBundle {
                        texture: texture_handle_body.clone(),
                        transform: Transform::from_xyz(0.0, 250.0, 0.0),
                        ..Default::default()
                    },
                    ZLayer::Foreground(5),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            WobbleJoint::new(peasant_entity, 0.05, -0.05, 0.002, true),
                            SpatialBundle {
                                transform: Transform::from_xyz(0.0, 250.0, 0.0),
                                ..Default::default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                SpriteBundle {
                                    texture: texture_handle_head.clone(),
                                    transform: Transform::from_xyz(0.0, 200.0, 0.0),
                                    ..Default::default()
                                },
                                ZLayer::Foreground(7),
                            ));
                        });
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
                    parent.spawn((
                        SpriteBundle {
                            texture: texture_handle_legs.clone(),
                            transform: Transform::from_xyz(0.0, -150.0, 0.0),
                            ..Default::default()
                        },
                        ZLayer::Foreground(6),
                    ));
                });
        })
        .id();

    let wobble_point_bottom = commands
        .spawn((
            WobbleJoint::new(peasant_entity, 0.2, -0.2, 0.009, false),
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: texture_handle_legs.clone(),
                    transform: Transform::from_xyz(0.0, -150.0, 0.0),
                    ..Default::default()
                },
                ZLayer::Foreground(4),
            ));
        })
        .id();

    let shadow_entity = commands
        .spawn((
            SpriteBundle {
                texture: shadow.clone(),
                transform: Transform::from_xyz(0.0, -300.0, 0.0),
                ..Default::default()
            },
            ZLayer::Foreground(3),
        ))
        .id();

    commands.entity(peasant_entity).push_children(&[
        wobble_point_top,
        wobble_point_bottom,
        shadow_entity,
    ]);
}

// Peasants stand around, when charles enters their cone of vision they give chase until charles is lost
