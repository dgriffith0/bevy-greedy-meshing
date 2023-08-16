mod camera;
mod greedy_mesher;
mod inputs;
pub mod raycast;
mod spawner;
mod ui;
mod voxel;

use bevy::{pbr::wireframe::WireframePlugin, prelude::*};
use bevy_mod_picking::DefaultPickingPlugins;
// use bevy_rapier3d::{
//     plugin::{NoUserData, RapierPhysicsPlugin},
//     prelude::RapierDebugRenderPlugin,
// };
use camera::GameCameraPlugin;
use greedy_mesher::GreedyMeshPlugin;
use inputs::InputsPlugin;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};
use spawner::SpawnerPlugin;
use ui::GameUiPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DefaultPickingPlugins,
            // RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            WireframePlugin,
            LookTransformPlugin,
            OrbitCameraPlugin::default(),
            GameCameraPlugin,
            GameUiPlugin,
            InputsPlugin,
            GreedyMeshPlugin,
            SpawnerPlugin,
        ))
        .run();
}
