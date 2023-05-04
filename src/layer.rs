use bevy::prelude::*;
use bevy_rapier2d::{parry::query::details::closest_points_line_line, na::ComplexField};

#[derive(Component, Clone, Copy, Debug)]
pub enum ZLayer {
    Background(i8),
    Foreground(i8),
}

pub fn update_z_coordinate_based_on_layer(
    mut query: Query<
        (&mut Transform, &GlobalTransform, &ZLayer),
        Or<(Changed<GlobalTransform>, Changed<ZLayer>)>,
    >,
) {
    for (mut transform, g_transform, layer) in query.iter_mut() {
        let forward_z = g_transform.forward().z;

        if forward_z.is_nan() || forward_z.is_infinite() || forward_z.abs() > 1.0 {
            continue;
        }
        transform.translation.z = match *layer {
            ZLayer::Background(order_in_layer) => {
                -forward_z * 5.0 + order_in_layer as f32 / 1000.
            }
            ZLayer::Foreground(order_in_layer) => {
                -forward_z * (10.0 + order_in_layer as f32 / 1000.)
            }
        }
    }
}

pub fn check_all_sprites_have_z_layer(query: Query<Entity, (With<Sprite>, Without<ZLayer>)>) {
    for id in query.iter() {
        println!("WARNING: {} doesn't have a ZLayer assigned", id.index());
    }
}
