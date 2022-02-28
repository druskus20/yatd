use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use smooth_bevy_cameras::LookTransformPlugin;
use yatd_lib::game_state::GameState;

// Consider using
// https://github.com/NiklasEi/bevy_asset_loader#:~:text=README.md-,Bevy%20asset%20loader,The%20trait%20can%20be%20derived.

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Yatd".to_string(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LookTransformPlugin)
        .add_plugin(TweeningPlugin) // TODO: Maybe fork to support conditional enabling. Should be ok for now.
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(yatd_lib::game_state::GameStatePlugin)
        .add_startup_system(setup.system())
        .add_startup_system(yatd_lib::env::load_assets)
        .add_plugin(yatd_lib::start_menu::StartMenuPlugin::run_in_state(
            GameState::StartMenu,
        ))
        .add_plugin(yatd_lib::camera::CameraPlugin::run_in_state(
            GameState::Defense,
        ))
        //.add_plugin(yatd_lib::camera::CameraPlugin::new())
        .add_plugin(yatd_lib::map::MapPlugin::run_in_state(GameState::Defense))
        .add_plugin(yatd_lib::tower::TowerPlugin::run_in_state(
            GameState::Defense,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 200.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_translation(Vec3::new(0.0, -0.5, 0.0)),
        ..Default::default()
    });

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            color: Color::rgba(0.8, 0.8, 0.8, 1.0),
            shadows_enabled: true,
            illuminance: 10000.0,
            ..Default::default()
        },
        transform: Transform {
            rotation: Quat::from_axis_angle(Vec3::new(0.1, 0.5, 0.9), -84.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Point light (Sun)
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            intensity: 100000.0,
            radius: 1000000.0,
            range: 1000000.0,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(10.0, 55.0, 20.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
