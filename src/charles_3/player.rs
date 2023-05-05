use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

#[derive(Component)]
pub struct Player;

pub fn create_player(commands: &mut Commands, assets: &Res<AssetServer>) {
    let entity = commands
        .spawn((
            Player,
            Velocity::zero(),
            SpatialBundle {
                transform: Transform::from_scale(Vec3 {
                    x: 0.1,
                    y: 0.1,
                    z: 0.1,
                })
                .with_translation(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                }),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((SpriteBundle {
                texture: assets.load("charles_3.png"),
                ..Default::default()
            },));
        })
        .id();
    commands.entity(entity);
}

pub fn move_with_velocity(mut transforms: Query<(&mut Transform, &mut Velocity)>) {
    transforms.iter_mut().for_each(|(mut trans, v)| {
        trans.translation += v.linvel.extend(0.0);
    });
}
