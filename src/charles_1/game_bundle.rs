use std::time::Duration;

use bevy::prelude::*;

use crate::{despawn_all, AppState, DeleteOnSceneChange};

use super::{
    character::{check_peasant_takes_charles, Charles1, Charles1Arm},
    falling_sprite::{caluculate_falling_sprites, FallingSprite},
    kills_required::{track_kills, PeasantKilled, TotalPeasantsKilled},
    peasant::{
        add_velocity_towards_charles, destroy_peasant, periodically_spawn_peasants, PeasantTimer,
    },
    ui::health_and_kills_needed_ui,
    wobble_joint::WobbleJointPlugin,
    *,
};

impl Plugin for Charles1Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(Charles1)
            .add_plugin(WobbleJointPlugin)
            .register_type::<FallingSprite>()
            .register_type::<CharlesVelocity>()
            .register_type::<Charles1Arm>()
            .init_resource::<TotalPeasantsKilled>()
            .add_event::<PeasantKilled>()
            .insert_resource(ClearColor(make_colour(BACKGROUND_COLOUR)))
            .insert_resource(PeasantTimer(Timer::new(
                Duration::from_secs_f32(5.6),
                TimerMode::Repeating,
            )))
            .add_system(load_charles_1.in_schedule(OnEnter(AppState::Charles1)))
            .add_system(despawn_all::<DeleteOnSceneChange>.in_schedule(OnExit(AppState::Charles1)))
            .add_systems(
                (
                    take_user_input,
                    normalize_z_level::normalize_z_level,
                    destroy_peasant,
                    periodically_spawn_peasants,
                    camera_follows_charles,
                    check_peasant_takes_charles,
                    add_velocity_towards_charles,
                    health_and_kills_needed_ui,
                    track_kills,
                )
                    .in_set(OnUpdate(AppState::Charles1)),
            )
            .add_systems(
                (
                    move_with_velocity,
                    caluculate_falling_sprites.before(destroy_peasant),
                )
                    .in_set(OnUpdate(AppState::Charles1))
                    .in_schedule(CoreSchedule::FixedUpdate),
            );
    }
}
