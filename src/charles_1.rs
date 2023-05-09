use crate::bevy_tiling_background::*;
use bevy::{prelude::*, sprite::*};

use self::{
    character::{create_charles_1, Charles1},
    cromwell::spawn_cromwell,
    kills_required::TotalPeasantsKilled,
    peasant::{spawn_peasant, FaceResources},
    ui::CrownLost,
};

mod character;
mod cromwell;
mod falling_sprite;
mod game_bundle;
mod kills_required;
mod normalize_z_level;
mod peasant;
mod ui;
mod wobble_joint;

pub struct Charles1Plugin;

#[derive(Component)]
pub struct Shadow;

const TOTAL_KILL_REQUIRED_TO_BEAT_LEVEL: usize = 50;
const PEASANT_MAX_HEALTH: usize = 1;

fn load_charles_1(
    mut commands: Commands,
    mut back_materials: ResMut<Assets<BackgroundMaterial>>,
    asset_server: Res<AssetServer>,
    mut required_kills: ResMut<TotalPeasantsKilled>,
    mut crown_lost: ResMut<CrownLost>,
) {
    let grass_image = asset_server.load("grass.png");
    commands.spawn((
        BackgroundImageBundle::from_image(grass_image, back_materials.as_mut())
            .at_z_layer(0.1)
            .with_movement_scale(0.5),
        crate::DeleteOnSceneChange,
    ));

    // create face image resource
    let happy_head: Handle<Image> = asset_server.load("peasant_head_happy.png");
    let shocked_head: Handle<Image> = asset_server.load("peasant_head_shocked.png");
    commands.insert_resource(FaceResources {
        happy_head,
        shocked_head,
    });

    // king
    create_charles_1(&mut commands, &asset_server);

    // spawn_cromwell(&mut commands, &asset_server);

    spawn_peasant(&mut commands, &asset_server, Vec2::new(100.0, -600.0));

    required_kills.0.clear();
    crown_lost.0.reset();
}

#[derive(Component)]
pub struct Circle;

#[derive(PartialEq, Eq, Clone, Copy, Reflect)]
pub enum FacingDirection {
    Left,
    Right,
}

#[derive(Component, Reflect)]
pub struct CharlesVelocity {
    can_change_facing_direction: bool,

    value: Vec2,
    facing: FacingDirection,
}

impl CharlesVelocity {
    fn new(can_change_facing_direction: bool) -> CharlesVelocity {
        CharlesVelocity {
            value: Vec2 { x: 0.0, y: 0.0 },
            can_change_facing_direction,
            facing: FacingDirection::Left,
        }
    }
}

fn process_userinput(
    key_code: &Input<KeyCode>,
    pos_codes: Vec<KeyCode>,
    neg_codes: Vec<KeyCode>,
) -> Option<f32> {
    let mut val = None;
    for pos_key in pos_codes.iter() {
        if key_code.just_released(*pos_key) && !key_code.any_pressed(neg_codes.clone()) {
            return Some(0.0);
        } else if key_code.just_pressed(*pos_key) {
            val = Some(1.0);
        }
    }
    for neg_key in neg_codes.iter() {
        if key_code.just_released(*neg_key) && !key_code.any_pressed(pos_codes.clone()) {
            return Some(0.0);
        } else if key_code.just_pressed(*neg_key) {
            val = Some(-1.0);
        }
    }

    return val;
}

fn take_user_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut velocities: Query<&mut CharlesVelocity, With<Charles1>>,
) {
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

fn move_with_velocity(mut transforms: Query<(&mut Transform, &mut CharlesVelocity)>) {
    for (mut trans, mut vel) in transforms.iter_mut() {
        trans.translation += (vel.value.try_normalize().unwrap_or_default() * 3.6).extend(0.0);
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

fn camera_follows_charles(
    charles: Query<&Transform, (With<Charles1>, Without<Camera2d>)>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Charles1>)>,
) {
    let mut cam_trans = camera.single_mut();
    let mut new_trans = charles.single().translation;
    new_trans.z = cam_trans.translation.z;
    new_trans.y += 200.;
    cam_trans.translation = new_trans;
}

// Colours are specified as CSS-style hex strings so that we can use VSCode's colour picker.
// This requires downloading the `AntiAntiSepticeye.vscode-color-picker` extension,
// and adding `"vscode-color-picker.languages": ["rust"]` to `settings.json`.
fn make_colour(s: &str) -> Color {
    return Color::hex(s).unwrap_or(Color::BLACK);
}
const BACKGROUND_COLOUR: &str = "#004DB3";
const CIRCLE_COLOUR: &str = "#FF00FF";
