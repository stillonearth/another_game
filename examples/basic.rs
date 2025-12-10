use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

fn main() {
    App::new()
        // Add Bevy's default plugins
        .add_plugins(DefaultPlugins)
        // Add the bevy_ecs_tiled plugin.
        // bevy_ecs_tilemap::TilemapPlugin will be added automatically if needed
        .add_plugins(TiledPlugin::default())
        // Add your startup system and run the app
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn a 2D camera
    commands.spawn(Camera2d);

    // Load a map asset and retrieve its handle
    let map_handle: Handle<TiledMapAsset> = asset_server.load("sk.tmx");

    // Spawn a new entity with the TiledMap component
    commands.spawn(TiledMap(map_handle));
}