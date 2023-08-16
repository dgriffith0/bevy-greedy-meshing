use bevy::prelude::*;
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(8.0, -1.0, 8.0),
        ..Default::default()
    });

    commands
        .spawn(Camera3dBundle::default())
        .insert((OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(28.0, 45.0, 28.0),
            Vec3::new(8., 8., 8.),
            Vec3::Y,
        ),));
}
