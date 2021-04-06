use crate::chunk_mesh_builder::ChunkMeshBuilder;
use bevy::prelude::Mesh;

pub struct Chunk {
    blocks: [[[u32; 16]; 256]; 16],
    mesh_builder: ChunkMeshBuilder,
}

impl Chunk {
    pub fn new() -> Self {
        let mut blocks = [[[0u32; 16]; 256]; 16];
        for x in 0..16usize {
            for y in 0..64usize {
                for z in 0..16usize {
                    blocks[x][y][z] = 1;
                }
            }
        }

        Chunk {
            blocks,
            mesh_builder: ChunkMeshBuilder::new()
        }
    }

    pub fn build_mesh(mut self) -> Mesh {
        for x in 0..16usize {
            for y in 0..256usize {
                for z in 0..16usize {
                    let val = &mut self.blocks[x][y][z];
                    if *val == 0 {
                        continue;
                    }

                    let coord = [x as u8, y as u8, z as u8];
                    if x == 0 || self.blocks[x - 1][y][z] == 0 {
                        self.mesh_builder.add_face(coord, 2);
                    }

                    if x == 15 || self.blocks[x + 1][y][z] == 0 {
                        self.mesh_builder.add_face(coord, 3);
                    }

                    if y == 0 || self.blocks[x][y - 1][z] == 0 {
                        self.mesh_builder.add_face(coord, 5);
                    }

                    if y == 255 || self.blocks[x][y + 1][z] == 0 {
                        self.mesh_builder.add_face(coord, 0);
                    }

                    if z == 0 || self.blocks[x][y][z - 1] == 0 {
                        self.mesh_builder.add_face(coord, 1);
                    }

                    if z == 15 || self.blocks[x][y][z + 1] == 0 {
                        self.mesh_builder.add_face(coord, 4);
                    }
                }
            }
        }

        self.mesh_builder.build()
    }
}