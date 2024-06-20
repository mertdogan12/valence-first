use valence::prelude::*;
use valence::spawn::IsFlat;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, init_clients)
        .run();
}

fn init_clients(
    mut clients: Query<
        (
            Entity,
            &mut Client,
            &mut VisibleChunkLayer,
            &mut IsFlat,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
    mut commands: Commands,
) {
    for (entity, mut client, mut visible_chunk_layer, mut is_flat, mut game_mode) in &mut clients {
        visible_chunk_layer.0 = entity;
        is_flat.0 = true;
        *game_mode = GameMode::Creative;

        client.send_chat_message("Hallo World!");

        let layer: ChunkLayer = ChunkLayer::new(ident!("overworld"), &dimensions, &biomes, &server);

        commands.entity(entity).insert(layer);
    }
}
