use bevy::{prelude::*, DefaultPlugins};

use crate::{despawn_all, AppState};

use super::*;

impl Plugin for Charles1Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(make_colour(BACKGROUND_COLOUR)))
            .add_system(load_charles_1.in_schedule(OnEnter(AppState::Charles1)))
            .add_system(despawn_all::<Circle>.in_schedule(OnExit(AppState::Charles1)))
            .add_systems(
                (take_user_input, move_circle, grow_circle).in_set(OnUpdate(AppState::Charles1)),
            );
    }
}