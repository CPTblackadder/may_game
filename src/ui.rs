use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::*, egui};

use crate::AppState;

pub(crate) fn scene_changer_ui(
    mut egui_contexts: EguiContexts,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    egui::Window::new("Scene Changer").show(egui_contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut selected = (*current_state).0.clone();
            egui::ComboBox::from_label("Select Scene")
                .selected_text(format!("{:?}", selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut selected, AppState::MainMenu, "Main Menu");
                    ui.selectable_value(&mut selected, AppState::Charles1, "Charles 1");
                });

            if selected != current_state.0 {
                next_state.set(selected);
            }
        });
    });
}
