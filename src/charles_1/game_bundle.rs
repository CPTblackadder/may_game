use bevy::prelude::*;

use crate::{despawn_all, AppState, DeleteOnSceneChange};

use super::{
    character::Charles1, falling_sprite::caluculate_falling_sprites, peasant::destroy_peasant,
    wobble_joint::WobbleJointPlugin, *,
};

impl Plugin for Charles1Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(Charles1)
            .add_plugin(WobbleJointPlugin)
            .insert_resource(ClearColor(make_colour(BACKGROUND_COLOUR)))
            .add_system(load_charles_1.in_schedule(OnEnter(AppState::Charles1)))
            .add_system(despawn_all::<DeleteOnSceneChange>.in_schedule(OnExit(AppState::Charles1)))
            .add_systems(
                (
                    take_user_input,
                    normalize_z_level::normalize_z_level,
                    destroy_peasant,
                )
                    .in_set(OnUpdate(AppState::Charles1)),
            )
            .add_systems(
                (caluculate_falling_sprites, move_with_velocity)
                    .in_set(OnUpdate(AppState::Charles1))
                    .in_schedule(CoreSchedule::FixedUpdate),
            );
    }
}
