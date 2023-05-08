use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct Config {
    pub speed: f32,
    pub keys: Keys,
}

pub struct Keys {
    pub left: &'static [KeyCode],
    pub right: &'static [KeyCode],
    pub jump: &'static [KeyCode],
}

pub fn create_player(commands: &mut Commands, assets: &Res<AssetServer>) {
    let entity = commands
        .spawn((
            Collider::cuboid(500.0, 50.0),
            Player,
            Velocity::zero(),
            Restitution::coefficient(0.0),
            RigidBody::Dynamic,
            GravityScale(0.1),
            SpriteBundle {
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
                texture: assets.load("charles_3.png"),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(entity);
}

pub fn move_with_velocity(mut transforms: Query<(&mut Transform, &mut Velocity)>) {
    transforms.iter_mut().for_each(|(mut trans, v)| {
        trans.translation += v.linvel.extend(0.0);
    });
}

pub fn handle_kb_input(
    config: Res<Config>,
    input: Res<Input<KeyCode>>,
    mut velocities: Query<&mut Velocity, With<Player>>,
) {
    velocities.iter_mut().for_each(|mut v| {
        for k in config.keys.left.iter() {
            if input.just_released(*k) && !config.keys.right.iter().any(|kr| input.pressed(*kr)) {
                v.linvel.x = 0.0;
            } else if input.just_pressed(*k) {
                v.linvel.x = -config.speed;
            }
        }
        for k in config.keys.right.iter() {
            if input.just_released(*k) && !config.keys.left.iter().any(|kl| input.pressed(*kl)) {
                v.linvel.x = 0.0;
            } else if input.just_pressed(*k) {
                v.linvel.x = config.speed;
            }
        }
        for k in config.keys.jump.iter() {
            if input.just_released(*k) {
                v.linvel.y = 0.0;
            } else if input.just_pressed(*k) {
                v.linvel.y = 6.0;
            }
        }
    });
}
