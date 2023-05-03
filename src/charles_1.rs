use bevy::{prelude::*, sprite::*};
use bevy_tiling_background::*   ;

use self::character::create_charles_1;

mod character;
mod game_bundle;
mod peasant;

pub struct Charles1Plugin;

fn load_charles_1(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut back_materials: ResMut<Assets<BackgroundMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let grass_image = asset_server.load("grass.png");
    
    commands.spawn(
        BackgroundImageBundle::from_image(grass_image, back_materials.as_mut(), meshes.as_mut())
            .at_z_layer(0.1),
    );
    // king
    create_charles_1(&mut commands, &asset_server);
}

#[derive(Component)]
pub struct Circle;

#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
}
impl Velocity {
    fn new() -> Velocity {
        Velocity {
            value: Vec2 { x: 0.0, y: 0.0 },
        }
    }

    fn is_left(&self) -> bool {
        return self.value.x <= 0.0;
    }
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

fn move_circle(mut transforms: Query<(&mut Transform, &Velocity)>) {
    for (mut trans, vel) in transforms.iter_mut() {
        trans.translation += vel.value.extend(0.0);
        if !vel.is_left() {
            trans.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        } else {
            trans.rotation = Quat::default();
        }
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
