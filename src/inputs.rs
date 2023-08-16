use bevy::{pbr::wireframe::WireframeConfig, prelude::*};

use crate::greedy_mesher::{generate_voxels, GreedyMeshConfig};

pub struct InputsPlugin;

impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_input);
    }
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut mesh_config: ResMut<GreedyMeshConfig>,
) {
    if keys.just_pressed(KeyCode::Space) {
        mesh_config.greedy = !mesh_config.greedy;
    }

    if keys.just_pressed(KeyCode::W) {
        wireframe_config.global = !wireframe_config.global;
    }

    if keys.just_pressed(KeyCode::R) {
        generate_voxels(&mut mesh_config);
    }

    if keys.just_pressed(KeyCode::D) {
        mesh_config.two_d = !mesh_config.two_d;
    }
}
