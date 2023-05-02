use std::default;

use bevy::{
    prelude::{shape::Circle, *},
    sprite::MaterialMesh2dBundle,
};

#[derive(Component)]
struct Peasant;

#[derive(Default)]
enum PeasantState {
    #[default]
    Idle,
    Tracking,
}

pub fn spawn_peasant(
    commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    location: Vec2,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(location.extend(3.0)),
            ..default()
        },
        Peasant,
    ));
}

// Peasants stand around, when charles enters their cone of vision they give chase until charles is lost
