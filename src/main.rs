use bevy::{
    prelude::*,
    window::{PresentMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_atmosphere::prelude::*;

pub const HEIGHT: f32 = 1080.0;
pub const WIDTH: f32 = 1920.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Skill Sphere".into(),
            resolution: (WIDTH, HEIGHT).into(),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }))
    .add_plugin(AtmospherePlugin)
    .add_startup_system(setup_scene)
    .add_startup_system(setup_camera)
    .add_plugin(WorldInspectorPlugin::new())
    .run();
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SkillSphere {

}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Pole {
    skillname: String,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let skillsphere = commands.spawn(SceneBundle {
        scene: asset_server.load("Star4.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    }).insert(Name::new("Sphere")).id();

    let pole = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::rgb(0.7, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    }).insert(Pole{
        skillname: "Fire".to_string(), 
    }).insert(Name::new("Pole")).id();

    // add the child to the parent
    commands.entity(skillsphere).push_children(&[pole]);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    }).insert(Name::new("Light"));

    commands.insert_resource(AtmosphereModel::new(Nishita {
        rayleigh_coefficient: Vec3::new(2e-5, 1e-5, 2e-5),
        ..default()
    }));
    
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 1.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        AtmosphereCamera::default(),
    )).insert(Name::new("camera"));
}
