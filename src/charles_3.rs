use self::player::{create_player, handle_kb_input, move_with_velocity, Config, Keys};
use crate::{bevy_tiling_background::*, despawn_all, AppState, DeleteOnSceneChange};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod player;

pub struct Charles3Plugin;
impl Plugin for Charles3Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CONFIG)
            .add_system(setup_physics.in_schedule(OnEnter(AppState::Charles3)))
            .add_system(init.in_schedule(OnEnter(AppState::Charles3)))
            .add_system(handle_kb_input.in_set(OnUpdate(AppState::Charles3)))
            .add_system(move_with_velocity.in_schedule(CoreSchedule::FixedUpdate))
            .add_system(despawn_all::<DeleteOnSceneChange>.in_schedule(OnExit(AppState::Charles1)));
    }
}

fn init(
    mut commands: Commands,
    mut bg_materials: ResMut<Assets<BackgroundMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        BackgroundImageBundle::from_image(
            asset_server.load("coronation_route.png"),
            bg_materials.as_mut(),
        ),
        DeleteOnSceneChange,
    ));
    create_player(&mut commands, &asset_server);
}

fn setup_physics(mut commands: Commands) {
    commands
        .spawn((Collider::cuboid(500.0, 50.0), DeleteOnSceneChange))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.0))
        .insert(ColliderMassProperties::Density(2.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)));
}

const CONFIG: player::Config = Config {
    speed: 3.0,
    keys: Keys {
        left: &[KeyCode::Left],
        right: &[KeyCode::Right],
        jump: &[KeyCode::Space],
    },
};
