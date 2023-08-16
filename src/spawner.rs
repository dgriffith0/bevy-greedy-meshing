use crate::greedy_mesher::{GreedyMeshConfig, VoxelMarker};
use crate::voxel::Voxels;
use bevy::prelude::*;
use bevy_mod_picking::prelude::{Click, On, Pointer, PointerButton, RaycastPickTarget};
use bevy_mod_picking::PickableBundle;
// use bevy_rapier3d::prelude::Collider;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_config_change);
    }
}

fn on_config_change(
    mesh_config: Res<GreedyMeshConfig>,
    query: Query<Entity, With<VoxelMarker>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if mesh_config.is_changed() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        if mesh_config.greedy {
            let voxels = if mesh_config.two_d {
                &mesh_config.greedy_blocks_2d
            } else {
                &mesh_config.greedy_blocks_3d
            };
            spawn_cubes(&mut commands, &mut meshes, &mut materials, voxels);
        } else {
            let voxels = if mesh_config.two_d {
                &mesh_config.standard_blocks_2d
            } else {
                &mesh_config.standard_blocks_3d
            };
            spawn_cubes(&mut commands, &mut meshes, &mut materials, voxels);
        }
    }
}
fn spawn_cubes(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    voxels: &Voxels,
) {
    (0..voxels.len()).for_each(|x| {
        (0..voxels[x].len()).for_each(|y| {
            (0..voxels.len()).for_each(|z| {
                if voxels[x][y][z].size.is_some() {
                    let size = voxels[x][y][z].size.unwrap();
                    spawn_cube(
                        commands,
                        materials,
                        get_cube_mesh(
                            (
                                x as f32,
                                y as f32,
                                z as f32,
                                size.0 as f32,
                                size.1 as f32,
                                size.2 as f32,
                            ),
                            meshes,
                        ),
                        voxels[x][y][z].color(),
                    );

                    commands.spawn((
                        PbrBundle {
                            transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
                            ..Default::default()
                        },
                        PickableBundle::default(),
                        RaycastPickTarget::default(),
                        On::<Pointer<Click>>::target_commands_mut(|click, target_commands| {
                            println!("Clicked the invisible cube");
                            // if click.button == PointerButton::Primary {
                            //     target_commands.despawn();
                            // }
                        }),
                    ));
                }
            });
        });
    });
}

fn get_cube_mesh(
    coords: (f32, f32, f32, f32, f32, f32),
    meshes: &mut Assets<Mesh>,
) -> Handle<Mesh> {
    // x,z is the bottom left corner, if the size is 1, we need to add size.
    // if the size is 2, then our x,y is the bottom left corner of the furthest right voxel
    let adjacent_top_corner = Vec3::new(coords.0 + 0.5, coords.1 + 0.5, coords.2 + 0.5);

    let bottom_corner = Vec3::new(
        adjacent_top_corner.x - coords.3,
        adjacent_top_corner.y - coords.4,
        adjacent_top_corner.z - coords.5,
    );

    meshes.add(Mesh::from(shape::Box::from_corners(
        bottom_corner,
        adjacent_top_corner,
    )))
}

// fn spawn_cuboid(commands: &mut Commands, coords: (f32, f32, f32, f32, f32, f32)) {
//     let adjacent_top_corner = Vec3::new(coords.0 + 1.0, coords.1 + 1.0, coords.2 + 1.0);

//     // let bottom_corner = Vec3::new(
//     //     adjacent_top_corner.x - coords.3,
//     //     adjacent_top_corner.y - coords.4,
//     //     adjacent_top_corner.z - coords.5,
//     // );

//     commands.spawn(Collider::cuboid(
//         adjacent_top_corner.x,
//         adjacent_top_corner.y,
//         adjacent_top_corner.z,
//     ));
// }

fn spawn_cube(
    commands: &mut Commands,
    materials: &mut Assets<StandardMaterial>,
    cube_mesh: Handle<Mesh>,
    color: Color,
) {
    commands.spawn((
        PbrBundle {
            mesh: cube_mesh,
            material: materials.add(StandardMaterial {
                base_color: color,
                unlit: true,
                ..default()
            }),
            ..Default::default()
        },
        VoxelMarker,
    ));
}
