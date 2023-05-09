use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::*,
    egui::{ProgressBar, RichText},
};

use crate::AppState;

use super::{
    character::Charles1, kills_required::TotalPeasantsKilled, TOTAL_KILL_REQUIRED_TO_BEAT_LEVEL,
};

pub fn health_and_kills_needed_ui(
    mut contexts: EguiContexts,
    peasants_killed: Res<TotalPeasantsKilled>,
) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        let required_kills = TOTAL_KILL_REQUIRED_TO_BEAT_LEVEL;
        let current_kills = peasants_killed.0.len();
        let progress = current_kills as f32 / required_kills as f32;

        ui.add(ProgressBar::new(progress).text(format!("{}/{}", current_kills, required_kills)));
    });
}

#[derive(Resource, Default)]
pub struct CrownLost(pub Timer, bool);

pub fn crown_loss_timer(
    mut contexts: EguiContexts,
    charles: Query<&Charles1>,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<CrownLost>,
) {
    let charles = charles.single();
    if !charles.has_crown {
        if !timer.1 {
            timer.0.set_duration(Duration::from_secs(10));
            timer.0.reset();
            timer.1 = true;
        }

        timer.0.tick(time.delta());

        if !timer.0.finished() {
            let ctx: &mut egui::Context = contexts.ctx_mut();
            egui::TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
                ui.add(
                    ProgressBar::new(timer.0.percent())
                        .text(format!("RECAPTURE THE CROWN QUICKLY!")),
                );
            });
        } else {
            next_state.set(AppState::Charles1);
        }
    }
}

pub fn win_ui(
    mut contexts: EguiContexts,
    key_code: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let ctx: &mut egui::Context = contexts.ctx_mut();
    egui::CentralPanel::default().show(&ctx, |ui| {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| {
                ui.label(RichText::new("You won!\nPress R to restart").size(50.0));
            },
        );
    });

    if key_code.just_pressed(KeyCode::R) {
        next_state.set(AppState::Charles1);
    }
}
