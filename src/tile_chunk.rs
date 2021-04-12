use bevy::{
    math::{Vec2, Vec4},
    render::{
        draw::Visible, mesh::VertexAttributeValues, pipeline::PrimitiveTopology,
        pipeline::RenderPipeline, render_graph::base::MainPass,
    },
};
use tiled::{LayerTile, Tileset};

use crate::{loader::TiledMapLoader, Map, TileMapChunk, TILE_MAP_PIPELINE_HANDLE};
use bevy::prelude::*;

#[derive(Debug)]
pub struct LayerChunk {
    pub position: Vec2,
    pub tiles: Vec<Vec<TileChunk>>,
}
#[derive(Bundle)]
pub struct ChunkBundle {
    pub map_parent: Handle<Map>, // tmp:chunks should be child entities of a toplevel map entity.
    pub chunk: TileMapChunk,
    pub main_pass: MainPass,
    pub material: Handle<ColorMaterial>,
    pub render_pipeline: RenderPipelines,
    pub visible: Visible,
    pub draw: Draw,
    pub mesh: Handle<Mesh>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for ChunkBundle {
    fn default() -> Self {
        Self {
            map_parent: Handle::default(),
            chunk: TileMapChunk::default(),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            draw: Default::default(),
            main_pass: MainPass,
            mesh: Handle::default(),
            material: Handle::default(),
            render_pipeline: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                TILE_MAP_PIPELINE_HANDLE.typed(),
            )]),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct TileChunk {
    pub tile_id: u32,
    pub pos: Vec2,
    pub vertex: Vec4,
    pub uv: Vec4,
    pub flip_d: bool,
    pub flip_h: bool,
    pub flip_v: bool,
}

impl TileChunk {
    pub fn from_layer_and_tileset(
        layer_tile: &LayerTile,
        tileset: &Tileset,
        chunk_pos: Vec2,
        vertex: Vec4,
    ) -> TileChunk {
        let tile_width = tileset.tile_width as f32;
        let tile_height = tileset.tile_height as f32;
        let tile_space = tileset.spacing as f32;
        let image = tileset.images.first().unwrap();
        let texture_width = image.width as f32;
        let texture_height = image.height as f32;
        let columns = ((texture_width + tile_space) / (tile_width + tile_space)).floor(); // account for no end tile

        let tile =
            (TiledMapLoader::remove_tile_flags(layer_tile.gid) as f32) - tileset.first_gid as f32;

        // This calculation is much simpler we only care about getting the remainder
        // and multiplying that by the tile width.
        let sprite_sheet_x: f32 =
            ((tile % columns) * (tile_width + tile_space) - tile_space).floor();

        // Calculation here is (tile / columns).round_down * (tile_space + tile_height) - tile_space
        // Example: tile 30 / 28 columns = 1.0714 rounded down to 1 * 16 tile_height = 16 Y
        // which is the 2nd row in the sprite sheet.
        // Example2: tile 10 / 28 columns = 0.3571 rounded down to 0 * 16 tile_height = 0 Y
        // which is the 1st row in the sprite sheet.
        let sprite_sheet_y: f32 =
            (tile / columns).floor() * (tile_height + tile_space) - tile_space;

        // Calculate UV:
        let start_u: f32 = sprite_sheet_x / texture_width;
        let end_u: f32 = (sprite_sheet_x + tile_width) / texture_width;
        let start_v: f32 = sprite_sheet_y / texture_height;
        let end_v: f32 = (sprite_sheet_y + tile_height) / texture_height;

        TileChunk {
            tile_id: layer_tile.gid,
            pos: chunk_pos.clone(),
            vertex: vertex.clone(),
            uv: Vec4::new(start_u, start_v, end_u, end_v),
            flip_d: layer_tile.flip_d,
            flip_h: layer_tile.flip_h,
            flip_v: layer_tile.flip_v,
        }
    }
}
