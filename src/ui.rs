use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::*, egui};

use crate::AppState;

pub(crate) fn scene_changer_ui<T>(
    mut egui_contexts: EguiContexts,
    current_state: Res<State<T>>,
    mut next_state: ResMut<NextState<T>>,
) where
    T: States + SelectableState,
{
    egui::Window::new(format!("Scene Changer {}", T::get_type_name())).show(
        egui_contexts.ctx_mut(),
        |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut selected = (*current_state).0.clone();
                egui::ComboBox::from_label("Select Scene")
                    .selected_text(format!("{:?}", selected))
                    .show_ui(ui, |ui| {
                        for (value, name) in T::get_states() {
                            ui.selectable_value(&mut selected, value, name);
                        }
                    });

                if selected != current_state.0 {
                    next_state.set(selected);
                }
            });
        },
    );
}

pub trait SelectableState
where
    Self: Sized,
{
    fn get_states() -> Vec<(Self, &'static str)>;
    fn get_type_name() -> &'static str;
}
