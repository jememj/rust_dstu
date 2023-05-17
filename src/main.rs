use bevy::prelude::*;

use std::f32::consts::TAU;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_cube)
        .run();
}

#[derive(Component)]
struct Rotatable {
    speed: f32,
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(5.0, 255.0, 255.0).into()),
        ..default()
    });
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(109.0, 101.0, 82.0).into()),
        // scene: asset_server.load("cube.gltf#Scene0"),
        // transform: Transform::from_xyz(0.0, 0.5, 0.0),
        transform: Transform::from_translation(Vec3::ZERO),
        // material: materials.add(Color::rgb(1., 1., 1.).into()),
        ..default()
    }, Rotatable { speed: 0.3 }));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn rotate_cube(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, cube) in &mut cubes {
        transform.rotate_y(cube.speed * TAU * timer.delta_seconds());
    }
}