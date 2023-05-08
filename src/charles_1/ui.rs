use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::*, egui::ProgressBar};

use super::{kills_required::TotalPeasantsKilled, TOTAL_KILL_REQUIRED_TO_BEAT_LEVEL};

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
