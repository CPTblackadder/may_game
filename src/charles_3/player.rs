use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_player(commands: &mut Commands, assets: &Res<AssetServer>) {
    let entity = commands
        .spawn((
            Player,
            Velocity::new(true),
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

#[derive(Component)]
pub struct Velocity {
    can_change_facing_direction: bool,
    pub value: Vec2,
    facing: FacingDirection,
}
impl Velocity {
    fn new(can_change_facing_direction: bool) -> Velocity {
        Velocity {
            value: Vec2 { x: 0.0, y: 0.0 },
            can_change_facing_direction,
            facing: FacingDirection::Left,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FacingDirection {
    Left,
    Right,
}

pub fn move_with_velocity(mut transforms: Query<(&mut Transform, &mut Velocity)>) {
    for (mut trans, mut vel) in transforms.iter_mut() {
        trans.translation += vel.value.extend(0.0);
        if vel.can_change_facing_direction {
            if vel.value.x < 0. && vel.facing == FacingDirection::Right {
                vel.facing = FacingDirection::Left;
                trans.scale.x = -trans.scale.x;
            } else if vel.value.x > 0. && vel.facing == FacingDirection::Left {
                vel.facing = FacingDirection::Right;
                trans.scale.x = -trans.scale.x;
            }
        }
    }
}
