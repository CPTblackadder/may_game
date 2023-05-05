mod bevy_tiling_background;
mod charles_1;
mod cursor_position;
mod ui;

use crate::bevy_tiling_background::*;
use bevy::{
    prelude::*,
    render::render_resource::{AddressMode, SamplerDescriptor},
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use charles_1::Charles1Plugin;
use ui::scene_changer_ui;

#[derive(Component)]
struct DeleteOnSceneChange;

fn main() {
    App::new()
        .add_state::<AppState>()
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: SamplerDescriptor {
                // Added from https://github.com/bevyengine/bevy/issues/399 to fix the back ground grass texture
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                address_mode_w: AddressMode::Repeat,
                ..Default::default()
            },
        }))
        .add_plugin(TilingBackgroundPlugin::<BackgroundMaterial>::default())
        .add_plugin(Charles1Plugin)
        .add_plugin(EguiPlugin)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(scene_changer_ui)
        .add_startup_system(common_start_up)
        .add_system(cursor_position::cursor_position)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

fn common_start_up(mut commands: Commands) {
    let mut cam_bundle = Camera2dBundle::default();
    cam_bundle.transform.scale = Vec3::new(1.2, 1.2, 1.);
    commands.spawn(cam_bundle);
}

fn despawn_all<T>(mut commands: Commands, query: Query<Entity, With<T>>)
where
    T: Component,
{
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    MainMenu,
    #[default]
    Charles1,
}
