use bevy::prelude::*;

pub struct GameUiPlugin;
#[derive(Component)]
struct VertexCounter;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_vertex_count);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "FPS: 0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ),
        VertexCounter,
    ));
}

fn update_vertex_count(
    mut query: Query<&mut Text, With<VertexCounter>>,
    meshes: Res<Assets<Mesh>>,
    meshed_entities: Query<(&Handle<Mesh>, Option<&ComputedVisibility>)>,
) {
    for mut text in &mut query {
        let vertex_count: usize = meshed_entities
            .iter()
            .filter(|(_, visibility)| {
                if visibility.is_some() {
                    visibility.unwrap().is_visible()
                } else {
                    false
                }
            })
            .filter_map(|(mesh, _)| meshes.get(mesh))
            .map(|mesh| mesh.count_vertices())
            .sum();
        text.sections[0].value = format!("Vertices: {}", vertex_count);
    }
}
