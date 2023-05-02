use std::cmp::max;

use bevy::prelude::*;

use super::Velocity;

#[derive(Component)]
pub struct Charles1;

impl Plugin for Charles1 {
    fn build(&self, app: &mut App) {
        app.add_system(raise_charles_1_arm);
    }
}

#[derive(Component)]
struct Charles1Arm;

pub fn create_charles_1(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let texture_handle_arm: Handle<Image> = asset_server.load("charles_1_arm.png");
    let texture_handle_top: Handle<Image> = asset_server.load("charles_1_top.png");
    let texture_handle_bottom: Handle<Image> = asset_server.load("charles_1_bottom.png");

    let top_entity = commands
        .spawn((SpriteBundle {
            texture: texture_handle_top.clone(),
            transform: Transform::from_xyz(50.0, 20.0, 5.0),

            ..Default::default()
        },))
        .id();

    let bottom_entity = commands
        .spawn((SpriteBundle {
            texture: texture_handle_bottom.clone(),
            transform: Transform::from_xyz(-62.1, -587.7, 6.0),
            ..Default::default()
        },))
        .id();

    let wobble_point_arm = commands
        .spawn((
            Charles1Arm,
            SpatialBundle {
                transform: Transform::from_xyz(-60.0, 85.0, 5.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((SpriteBundle {
                texture: texture_handle_arm.clone(),
                transform: Transform::from_xyz(-152.3, -177.0, 0.0),
                ..Default::default()
            },));
        })
        .id();

    let charles_entity = commands
        .spawn((
            Charles1,
            Velocity::new(),
            SpatialBundle {
                ..Default::default()
            },
        ))
        .id();

    commands
        .entity(charles_entity)
        .push_children(&[wobble_point_arm, top_entity, bottom_entity]);
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
            new_angle = old_angle - (1.0 * timer.delta_seconds());
        } else {
            new_angle = old_angle + (3.0 * timer.delta_seconds());
        }

        arm.rotation = Quat::from_rotation_z(new_angle.clamp(-2.0, 0.0));
    }
}
