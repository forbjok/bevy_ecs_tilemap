use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};
use bevy_ecs_tilemap::prelude::*;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(12800.0, 12800.0, 1000.0 - 0.1),
        ..OrthographicCameraBundle::new_2d()
    });

    let texture_handle = asset_server.load("tiles.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map with (10 * 128) ^ 2 tiles or 1,638,400 tiles.
    // Be patient when running this example as meshing does not run on multiple CPU's yet..
    let mut map = Map::new(Vec2::new(10.0, 10.0).into(), Vec2::new(128.0, 128.0).into(), Vec2::new(16.0, 16.0), Vec2::new(96.0, 256.0));
    let map_entity = commands.spawn()
        .insert(Transform::default())
        .id();
    map.build(&mut commands, &mut meshes, material_handle, map_entity);

    commands.entity(map_entity).insert(map);
}

fn main() {
    env_logger::Builder::from_default_env()
    .filter_level(log::LevelFilter::Info)
    .init();

    App::build()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Random Map Example"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TileMapPlugin)
        .add_startup_system(startup.system())
        .run();
}