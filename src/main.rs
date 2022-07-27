mod components;
mod helpers;
mod systems;

use bevy::{
    app::App,
    math::Vec3,
    prelude::{
        AssetServer, Commands, GlobalTransform, Msaa, OrthographicCameraBundle, Res, Transform,
    },
    render::render_resource::FilterMode,
    window::WindowDescriptor,
    DefaultPlugins,
};

use bevy_ecs_tilemap::{
    ChunkSize, HexType, LayerBuilder, LayerSettings, Map, MapQuery, MapSize, TextureSize, Tile,
    TileBundle, TilePos, TileSize, TilemapMeshType, TilemapPlugin,
};
use components::MainCamera;
use rand::{thread_rng, Rng};
use systems::{click_cell, wiggle};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Hexacells".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup)
        .add_system(helpers::camera::movement)
        .add_system(helpers::texture::set_texture_filters_to_nearest)
        // .add_system(click_cell)
        .add_system(wiggle)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    // .insert(Transform::from_scale(Vec3::new(0.5, 0.5, 1.)));

    let texture_handle = asset_server.load("hexagonz.png");

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let mut map_settings = LayerSettings::new(
        MapSize(2, 2),
        ChunkSize(64, 64),
        TileSize(71.0, 60.0),
        TextureSize(71.0, 180.0),
    );
    map_settings.mesh_type = TilemapMeshType::Hexagon(HexType::Column);
    map_settings.filter = FilterMode::Linear;

    let (mut layer_builder, layer_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, map_settings.clone(), 0u16, 0u16);
    map.add_layer(&mut commands, 0u16, layer_entity);

    // layer_builder.fill(
    //     TilePos(0, 0),
    //     TilePos(64, 64),
    //     Tile {
    //         texture_index: 0,
    //         ..Default::default()
    //     }
    //     .into(),
    // );
    // layer_builder.fill(
    //     TilePos(64, 0),
    //     TilePos(128, 64),
    //     Tile {
    //         texture_index: 1,
    //         ..Default::default()
    //     }
    //     .into(),
    // );
    // layer_builder.fill(
    //     TilePos(0, 64),
    //     TilePos(64, 128),
    //     Tile {
    //         texture_index: 2,
    //         ..Default::default()
    //     }
    //     .into(),
    // );
    // layer_builder.fill(
    //     TilePos(64, 64),
    //     TilePos(128, 128),
    //     Tile {
    //         texture_index: 0,
    //         ..Default::default()
    //     }
    //     .into(),
    // );

    let mut random = thread_rng();

    for x in 0..128 {
        for y in 0..128 {
            let position = TilePos(x, y);
            // Ignore errors for demo sake.
            let _ = layer_builder.set_tile(
                position,
                Tile {
                    texture_index: random.gen_range(0..3),
                    ..Default::default()
                }
                .into(),
            );
        }
    }

    map_query.build_layer(&mut commands, layer_builder, texture_handle.clone());

    // for z in 0..2 {
    //     let (mut layer_builder, layer_entity) =
    //         LayerBuilder::<TileBundle>::new(&mut commands, map_settings, 0u16, z + 1);
    //     map.add_layer(&mut commands, z + 1, layer_entity);

    //     let mut random = thread_rng();

    //     for _ in 0..100 {
    //         let position = TilePos(random.gen_range(0..128), random.gen_range(0..128));
    //         // Ignore errors for demo sake.
    //         let _ = layer_builder.set_tile(
    //             position,
    //             Tile {
    //                 texture_index: z + 1,
    //                 ..Default::default()
    //             }
    //             .into(),
    //         );
    //     }

    //     map_query.build_layer(&mut commands, layer_builder, texture_handle.clone());
    // }

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-48.0, -24.0, 0.0))
        .insert(GlobalTransform::default());
}
