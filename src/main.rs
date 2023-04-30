use bevy::{prelude::*, sprite::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(take_user_input)
        .add_system(move_circle)
        .add_system(grow_circle)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Circle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::FUCHSIA)),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        },
        Velocity {
            value: Vec2::new(0.0, 0.0),
        },
        Circle,
    ));
}

#[derive(Component)]
pub struct Circle;

#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
}

fn process_userinput(
    key_code: &Input<KeyCode>,
    pos_codes: Vec<KeyCode>,
    neg_codes: Vec<KeyCode>,
) -> Option<f32> {
    let mut val = None;
    for pos_key in pos_codes.iter() {
        if key_code.just_released(*pos_key) {
            return Some(0.0);
        } else if key_code.just_pressed(*pos_key) {
            val = Some(1.0);
        }
    }
    for neg_key in neg_codes.iter() {
        if key_code.just_released(*neg_key) {
            return Some(0.0);
        } else if key_code.just_pressed(*neg_key) {
            val = Some(-1.0);
        }
    }

    return val;
}

fn take_user_input(keyboard_input: Res<Input<KeyCode>>, mut velocities: Query<&mut Velocity>) {
    for mut vel in velocities.iter_mut() {
        vel.value.x = process_userinput(
            &*keyboard_input,
            [KeyCode::D, KeyCode::Right].into(),
            [KeyCode::A, KeyCode::Left].into(),
        )
        .unwrap_or(vel.value.x);
        vel.value.y = process_userinput(
            &*keyboard_input,
            [KeyCode::W, KeyCode::Up].into(),
            [KeyCode::S, KeyCode::Down].into(),
        )
        .unwrap_or(vel.value.y);
    }
}

fn move_circle(mut transforms: Query<(&mut Transform, &Velocity), With<Circle>>) {
    for (mut trans, vel) in transforms.iter_mut() {
        println!("Translate by {}", vel.value);
        trans.translation += vel.value.extend(0.0);
    }
}

fn grow_circle(mut transforms: Query<&mut Transform, With<Circle>>) {
    transforms.iter_mut().for_each(|mut trans| {
        trans.scale += Vec3::new(0.005, 0.005, 0.0);
    });
}
