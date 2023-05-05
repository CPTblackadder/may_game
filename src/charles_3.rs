use self::player::{create_player, handle_kb_input, move_with_velocity, Config, Keys};
use crate::{bevy_tiling_background::*, AppState};
use bevy::prelude::*;

mod player;

pub struct Charles3Plugin;
impl Plugin for Charles3Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CONFIG)
            .add_system(init.in_schedule(OnEnter(AppState::Charles3)))
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

const CONFIG: player::Config = Config {
    speed: 3.0,
    keys: Keys {
        left: &[KeyCode::Left],
        right: &[KeyCode::Right],
    },
};
