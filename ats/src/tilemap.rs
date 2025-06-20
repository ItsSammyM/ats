use bevy_ecs_tilemap::prelude::*;
use bevy::prelude::*;

pub(crate) fn startup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    array_texture_loader: Res<ArrayTextureLoader>
){
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let tilemap_entity = commands.spawn_empty().id();
    
    let map_size = TilemapSize{x: 64, y: 64};
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos{x, y};
            let tile_entity = commands
                .spawn(TileBundle{
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize{x: 16.0, y: 16.0};
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..default()
    });

    array_texture_loader.add(TilemapArrayTexture{
        texture: TilemapTexture::Single(asset_server.load("tiles.png")),
        tile_size,
        ..default()
    });
}