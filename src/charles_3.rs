use self::player::{create_player, move_with_velocity, Player, Velocity};
use crate::{bevy_tiling_background::*, AppState};
use bevy::prelude::*;

mod player;

pub struct Charles3Plugin;
impl Plugin for Charles3Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(init.in_schedule(OnEnter(AppState::Charles3)))
            .add_system(handle_kb_input.in_set(OnUpdate(AppState::Charles3)))
            .add_system(move_with_velocity.in_schedule(CoreSchedule::FixedUpdate));
    }
}

fn init(
    mut commands: Commands,
    mut bg_materials: ResMut<Assets<BackgroundMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(BackgroundImageBundle::from_image(
        asset_server.load("coronation_route.png"),
        bg_materials.as_mut(),
    ));
    create_player(&mut commands, &asset_server);
}

fn handle_kb_input(input: Res<Input<KeyCode>>, mut velocities: Query<&mut Velocity, With<Player>>) {
    let process_codes =
        |key_code: &Input<KeyCode>, pos_codes: Vec<KeyCode>, neg_codes: Vec<KeyCode>| {
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
        };
    for mut vel in velocities.iter_mut() {
        vel.value.x = process_codes(
            &*input,
            [KeyCode::D, KeyCode::Right].into(),
            [KeyCode::A, KeyCode::Left].into(),
        )
        .unwrap_or(vel.value.x);
        vel.value.y = process_codes(
            &*input,
            [KeyCode::W, KeyCode::Up].into(),
            [KeyCode::S, KeyCode::Down].into(),
        )
        .unwrap_or(vel.value.y);
        vel.value;
    }
}
