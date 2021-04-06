mod rendering_constants;
mod fly_cam;
mod chunk_mesh_builder;
mod chunk;

extern crate bevy;
extern crate bevy_rapier3d;

use bevy::prelude::*;
use bevy::render::pipeline::{PrimitiveTopology, RenderPipeline};
use bevy::render::mesh::Indices;
use crate::rendering_constants::*;
use crate::fly_cam::*;
use crate::chunk_mesh_builder::ChunkMeshBuilder;
use crate::chunk::Chunk;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 80.0, 4.0),
        ..Default::default()
    });

    let mut chunk=Chunk::new();
    let texture_handle=asset_server.load("dirt.png");
    let white_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        unlit: false,
        ..Default::default()
    });


    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(chunk.build_mesh()),
        material: white_material.clone(),
        transform: Transform::from_xyz(1.0, 0.5, 0.0),
        ..Default::default()
    });
}
