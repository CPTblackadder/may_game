mod bevy_tiling_background;
mod charles_1;
mod charles_3;
mod cursor_position;
pub mod fading_sprite;
mod ui;

use crate::bevy_tiling_background::*;
use bevy::{
    prelude::*,
    render::render_resource::{AddressMode, SamplerDescriptor},
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::{prelude::*, render::RapierDebugRenderPlugin};
use charles_1::Charles1Plugin;
use charles_3::Charles3Plugin;
use fading_sprite::fade_then_remove_fading_sprites;
use ui::{scene_changer_ui, SelectableState};

#[derive(Component)]
struct DeleteOnSceneChange;

fn main() {
    let mut app = App::new()
        .add_state::<AppState>()
        .add_state::<Charles1State>()
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
        .add_plugin(Charles3Plugin)
        .add_plugin(EguiPlugin)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(scene_changer_ui::<AppState>)
        .add_system(scene_changer_ui::<Charles1State>)
        .add_system(fade_then_remove_fading_sprites)
        .add_startup_system(common_start_up)
        // .add_system(cursor_position::cursor_position)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
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
pub enum AppState {
    MainMenu,
    #[default]
    Charles1,
    Charles3,
}

impl SelectableState for AppState {
    fn get_states() -> Vec<(Self, &'static str)> {
        vec![
            (Self::MainMenu, "Main Menu"),
            (Self::Charles1, "Charles 1"),
            (Self::Charles3, "Charles 3"),
        ]
    }

    fn get_type_name() -> &'static str {
        "AppState"
    }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum Charles1State {
    #[default]
    OpeningCinematic,
    Play,
    ClosingCinematic,
}

impl SelectableState for Charles1State {
    fn get_states() -> Vec<(Self, &'static str)> {
        vec![
            (Self::OpeningCinematic, "Opening Cinematic"),
            (Self::Play, "Play"),
            (Self::ClosingCinematic, "Closing Cinematic"),
        ]
    }

    fn get_type_name() -> &'static str {
        "Charles1State"
    }
}
