use std::{default, f32::consts::PI};

use bevy::{
    prelude::{shape::Circle, *},
    sprite::MaterialMesh2dBundle,
};

use super::{falling_sprite::FallingSprite, wobble_joint::WobbleJoint, Shadow, Velocity};

#[derive(Component)]
struct Peasant;

#[derive(Default)]
enum PeasantState {
    #[default]
    Idle,
    Tracking,
}

#[derive(Component)]
pub struct PeasantDie;

pub fn destroy_peasant(
    mut commands: Commands,
    peasants: Query<(Entity, &Children, &Transform), With<PeasantDie>>,
    parent_child: Query<&Children>,
    sprites: Query<(Entity, &Handle<Image>), (With<Sprite>, Without<Shadow>)>,
    images: Res<Assets<Image>>,
) {
    for (p, children, t) in peasants.iter() {
        let floor = t.translation.y;
        apply_falling_sprite_rec(
            floor,
            &mut commands,
            children,
            &parent_child,
            &sprites,
            &images,
        );
        // commands.entity(p).despawn_recursive();
    }
}

fn apply_falling_sprite_rec(
    floor: f32,
    mut commands: &mut Commands,
    children: &Children,
    parent_child: &Query<&Children>,
    sprites: &Query<(Entity, &Handle<Image>), (With<Sprite>, Without<Shadow>)>,
    images: &Res<Assets<Image>>,
) {
    for child in children {
        if let Ok(children) = parent_child.get(*child) {
            apply_falling_sprite_rec(floor, commands, children, parent_child, sprites, images)
        }
        if let Ok((entity, image_handle)) = sprites.get(*child) {
            let i = images.get(image_handle);
            let sprite_radious = if let Some(i) = i {
                (i.texture_descriptor.size.height as f32 + i.texture_descriptor.size.width as f32)
                    / 2.
            } else {
                0.0
            };

            commands
                .entity(entity)
                .insert(dbg!(FallingSprite {
                    floor,
                    rotation_speed: 0.001,
                    velocity: Vec2 { x: 0.5, y: 10.0 },
                    sprite_radious,
                }))
                .remove_parent_in_place();
                // .despawn_descendants();
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

    let mut peasant_transform: Transform = Transform::from_translation(location.extend(0.0));
    peasant_transform.scale = Vec3::new(0.2, 0.2, 1.0);

    let peasant_entity = commands
        .spawn((
            Peasant,
            PeasantDie,
            Velocity::new(false),
            SpatialBundle {
                transform: peasant_transform,
                ..Default::default()
            },
            crate::DeleteOnSceneChange,
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
                        .with_children(|parent| {
                            parent.spawn((SpriteBundle {
                                texture: texture_handle_head.clone(),
                                transform: Transform::from_xyz(0.0, 200.0, 0.1),
                                ..Default::default()
                            },));
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
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            Shadow,
        ))
        .id();

    commands.entity(peasant_entity).push_children(&[
        wobble_point_top,
        wobble_point_bottom,
        shadow_entity,
    ]);
}

// Peasants stand around, when charles enters their cone of vision they give chase until charles is lost
