use bevy::prelude::*;

use crate::DeleteOnSceneChange;

pub fn spawn_cromwell(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let cromwell_sprite: Handle<Image> = asset_server.load("cromwell_2.png");

    commands.spawn((
        SpriteBundle {
            texture: cromwell_sprite.clone(),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(0.5, 0.5, 1.0)),
            ..Default::default()
        },
        DeleteOnSceneChange,
        Name::new("Cromwell"),
    ));
}
