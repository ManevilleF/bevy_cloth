use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;
use bevy_silk::prelude::*;

mod camera_plugin;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ResourceInspectorPlugin::<ClothConfig>::new())
        .add_plugins(camera_plugin::CameraPlugin)
        .add_plugins(ClothPlugin)
        .insert_resource(ClothConfig {
            ..Default::default()
        })
        .add_systems(Startup, (spawn_cloth, setup))
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(DirectionalLightBundle::default());
    let mesh = meshes.add(shape::Cube::new(50.0).into());

    // Ground
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, -20.0, 0.0),
            ..Default::default()
        },
        Name::new("Ground"),
        Collider::cuboid(25.0, 25.0, 25.0),
    ));
}

fn spawn_cloth(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 5.0,
                    subdivisions: 10,
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(Color::YELLOW.into()),
            transform: Transform::from_xyz(0.0, 15.0, 0.0),
            ..Default::default()
        },
        ClothBuilder::new(),
        ClothInflator::new(0.7),
        ClothCollider {
            velocity_coefficient: 2.0,
            ..default()
        },
        Name::new("Balloon"),
    ));
}
