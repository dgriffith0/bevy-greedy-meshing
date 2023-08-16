// use bevy::prelude::*;
// use bevy::reflect::Reflect;
// use bevy_mod_raycast::*;

// use bevy_mod_raycast::print_intersections;

// #[derive(Reflect)]
// pub struct MyRaycastSet;

// pub struct RaycastPlugin;

// impl Plugin for RaycastPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugins(DefaultRaycastingPlugin::<MyRaycastSet>::default())
//             // You will need to pay attention to what order you add systems! Putting them in the wrong
//             // order can result in multiple frames of latency. Ray casting should probably happen near
//             // start of the frame. For example, we want to be sure this system runs before we construct
//             // any rays, hence the ".before(...)". You can use these provided RaycastSystem labels to
//             // order your systems with the ones provided by the raycasting plugin.
//             .add_systems(
//                 First,
//                 update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MyRaycastSet>),
//             )
//             .add_systems(Startup, (setup, print_intersections::<MyRaycastSet>));
//     }
// }
// // Update our `RaycastSource` with the current cursor position every frame.
// fn update_raycast_with_cursor(
//     mut cursor: EventReader<CursorMoved>,
//     mut query: Query<&mut RaycastSource<MyRaycastSet>>,
// ) {
//     // Grab the most recent cursor event if it exists:
//     let Some(cursor_moved) = cursor.iter().last() else { return };
//     for mut pick_source in &mut query {
//         pick_source.cast_method = RaycastMethod::Screenspace(cursor_moved.position);
//     }
// }

// // Set up a simple 3D scene
// fn setup(mut commands: Commands) {
//     // Overwrite the default plugin state with one that enables the debug cursor. This line can be
//     // removed if the debug cursor isn't needed as the state is set to default values when the
//     // default plugin is added.
//     commands.insert_resource(RaycastPluginState::<MyRaycastSet>::default().with_debug_cursor());
// }
