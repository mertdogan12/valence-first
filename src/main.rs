use valence::prelude::*;

const FLOOR_LEVEL: i32 = 40;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, init_clients)
        .run();
}

// Creates a floor
fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    let mut layer: LayerBundle =
        LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 { for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -10..10 {
        for x in -10..10 {
            layer.chunk.set_block([x, FLOOR_LEVEL, z], BlockState::COBBLESTONE);
        }
    }

    commands.spawn(layer);
}

fn init_clients(
    mut clients: Query<
        (
            &mut EntityLayerId,
            &mut Client,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        mut entity_layer_id,
        mut client,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut position,
        mut game_mode,
    ) in &mut clients
    {
        let layer = layers.single();

        entity_layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        position.set([0.0, f64::from(FLOOR_LEVEL) + 1.0, 0.0]);
        *game_mode = GameMode::Survival;

        client.send_chat_message("Hallo World!");
    }
}
