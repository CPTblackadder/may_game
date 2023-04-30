use bevy::{prelude::*, sprite::*};

mod character;
mod game_bundle;

pub struct Charles1Plugin;

fn load_charles_1(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Circle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(make_colour(CIRCLE_COLOUR))),
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
    let speed = 3.0;
    let mut val = None;
    for pos_key in pos_codes.iter() {
        if key_code.just_released(*pos_key) && !key_code.any_pressed(neg_codes.clone()) {
            return Some(0.0);
        } else if key_code.just_pressed(*pos_key) {
            val = Some(speed);
        }
    }
    for neg_key in neg_codes.iter() {
        if key_code.just_released(*neg_key) && !key_code.any_pressed(pos_codes.clone()) {
            return Some(0.0);
        } else if key_code.just_pressed(*neg_key) {
            val = Some(-speed);
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
        vel.value;
    }
}

fn move_circle(mut transforms: Query<(&mut Transform, &Velocity), With<Circle>>) {
    for (mut trans, vel) in transforms.iter_mut() {
        trans.translation += vel.value.extend(0.0);
    }
}

fn grow_circle(mut transforms: Query<&mut Transform, With<Circle>>) {
    transforms.iter_mut().for_each(|mut trans| {
        trans.scale += Vec3::new(0.005, 0.005, 0.0);
    });
}

// Colours are specified as CSS-style hex strings so that we can use VSCode's colour picker.
// This requires downloading the `AntiAntiSepticeye.vscode-color-picker` extension,
// and adding `"vscode-color-picker.languages": ["rust"]` to `settings.json`.
fn make_colour(s: &str) -> Color {
    return Color::hex(s).unwrap_or(Color::BLACK);
}
const BACKGROUND_COLOUR: &str = "#004DB3";
const CIRCLE_COLOUR: &str = "#FF00FF";