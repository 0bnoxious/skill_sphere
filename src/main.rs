use bevy::{
    prelude::*,
    window::{PresentMode}, 
    render::{
        mesh::{
            Indices, PrimitiveTopology,
            VertexAttributeValues, self,
        },
        render_resource::{AsBindGroup, ShaderRef},
    }, math::Vec3A,
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
    .add_startup_system(setup_skillsphere)
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
    skill_name: String,
}

fn setup_scene(
    mut commands: Commands,
) {
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

fn setup_skillsphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let tetra_lenght: f32 = 1.0;
    /*let mut tetra_vertices = vec![];


    tetra_vertices.push(Vec3::new(0.0, tetra_lenght * f32::sqrt(2./3.), 0.0)); //apex) 
    tetra_vertices.push(Vec3::new(tetra_lenght / f32::sqrt(3.), 0.0, 0.0));
    tetra_vertices.push(Vec3::new(0.0, -(1. / (2. * f32::sqrt(3.))), tetra_lenght / 2.));
    tetra_vertices.push(Vec3::new(0.0, -(1. / (2. * f32::sqrt(3.))), -(tetra_lenght / 2.)));*/

    let tetra_vertices = vec![
        Vec3::new(0.0, tetra_lenght * f32::sqrt(2./3.), 0.0), //apex
        Vec3::new(tetra_lenght / f32::sqrt(3.), 0.0, 0.0),
        Vec3::new(0.0, -(1. / (2. * f32::sqrt(3.))), tetra_lenght / 2.),
        Vec3::new(0.0, -(1. / (2. * f32::sqrt(3.))), -(tetra_lenght / 2.))
    ];

   /* // Triangle side #1
    tetra_indices.push(mesh::Indices::U32(vec![0, 2, 1]));
    // Triangle side #2
    tetra_indices.push(mesh::Indices::U32(vec![0, 2, 3]));
    // Triangle side #3
    tetra_indices.push(mesh::Indices::U32(vec![0, 3, 1]));
    // Triangle #4 bottom
    tetra_indices.push(mesh::Indices::U32(vec![1, 2, 3]));*/

    let tetra_indices = Indices::U32(vec![
        0, 2, 1,
        0, 3, 2,
        0, 3, 1,
        1, 2, 3
    ]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION,tetra_vertices);
    // A triangle using vertices 0, 2, and 1.
    // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
    //mesh.set_indices(Some(mesh::Indices::U32(vec![0, 2, 1])));
    mesh.set_indices(Some(tetra_indices));


    //mesh.set_indices(Some(tetra_indices.into_iter().map(Some).collect()));
    //let as_option: Vec<Option<Indices>> = tetra_indices.into_iter().map(Some).collect();
    //mesh.set_indices(as_option.into_iter().collect());

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    
    /*let skillsphere = commands.spawn(SceneBundle {
        scene: asset_server.load("Star.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    }).insert(Name::new("Sphere")).id();*/

    /*let pole = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::rgb(0.7, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    }).insert(Pole{
        skill_name: "Fire".to_string(), 
    }).insert(Name::new("Pole")).id();

    // add the child to the parent
    commands.entity(skillsphere).push_children(&[pole]);*/
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
