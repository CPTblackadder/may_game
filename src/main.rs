mod charles_1;
mod cursor_position;
mod ui;

use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use charles_1::Charles1Plugin;
use ui::scene_changer_ui;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(Charles1Plugin)
        .add_plugin(EguiPlugin)
        .add_plugin(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(common_start_up)
        .add_system(scene_changer_ui)
        .add_system(cursor_position::cursor_position)
        .run();
}

fn common_start_up(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_all<T>(mut commands: Commands, query: Query<Entity, With<T>>)
where
    T: Component,
{
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    Charles1,
}
