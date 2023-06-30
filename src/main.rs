use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{CursorGrabMode, PresentMode, WindowLevel},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
    .add_startup_system(setup_scene)
    .add_startup_system(setup_camera)
    .add_plugin(WorldInspectorPlugin::new())
    .run();
}

#[derive(Component)]
struct SkillSphere;

/*#[derive(Bundle)]
struct SpherePole{
    origin: SkillSphere,
    skillname: str,
    location: Vec3,

}*/

#[derive(Component)]
struct SphereCamera{
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

/// set up a simple 3D scene
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Icosphere::default().try_into().unwrap()),
        material: materials.add(Color::rgb(1.0, 0.1, 0.1).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(Name::new("Sphere"));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    }).insert(Name::new("Light"));
    
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
