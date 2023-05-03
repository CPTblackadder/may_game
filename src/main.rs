mod charles_1;
mod cursor_position;
mod layer;
mod ui;

use bevy::{
    prelude::*,
    render::render_resource::{AddressMode, SamplerDescriptor},
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_tiling_background::*;
use charles_1::Charles1Plugin;
use layer::{check_all_sprites_have_z_layer, update_z_coordinate_based_on_layer};
use ui::scene_changer_ui;

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
        .add_plugin(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(check_all_sprites_have_z_layer) // TODO remove for release, is debugging system
        .add_system(scene_changer_ui)
        .add_startup_system(common_start_up)
        .add_system(update_z_coordinate_based_on_layer.in_base_set(CoreSet::PostUpdate))
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
    MainMenu,
    #[default]
    Charles1,
}
