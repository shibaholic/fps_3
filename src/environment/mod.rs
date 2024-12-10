use avian3d::prelude::{Collider, ColliderConstructor, ColliderConstructorHierarchy, RigidBody};
use bevy::{color::palettes::tailwind, prelude::*};
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (spawn_world_model, spawn_lights))
        ;
    }
}

fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>
) {
    // let floor = meshes.add(Cuboid::new(20.0, 1.0, 20.0));
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let material = materials.add(Color::WHITE);

    // The world model camera will render the floor and the cubes spawned in this system.
    // Assigning no `RenderLayers` component defaults to layer 0.

    commands.spawn((
        SceneRoot(assets.load("character_controller_demo.glb#Scene0")),
        Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::PI * 0.5)),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        RigidBody::Static
    ));

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 4.0, -3.0),

        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic
    ));

    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(material),
        Transform::from_xyz(0.75, 4.0, 0.0),

        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic
    ));
}

fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::NEUTRAL_950),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, -0.75),
        // The light source illuminates both the world model and the view model.
        // RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            range: 50.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 15.0, 0.0),
    ));
}
