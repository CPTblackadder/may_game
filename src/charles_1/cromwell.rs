pub fn spawn_cromwell(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let cromwell_sprite: Handle<Image> = asset_server.load("cromwell_2.png");

    commands.spawn((
        SpriteBundle {
            texture: cromwell_sprite.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        Name::new("Cromwell"),
    ));
}
