use bevy::{pbr::NotShadowCaster, prelude::*, utils::FloatOrd};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
//use bevy::prelude::App::add_system;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH:  f32 = 1280.0;

#[derive(Resource)]
pub struct GameAssets{
    tower_base_scene: Handle<Scene>,
    generic_tower_scene: Handle<Scene>,
    tower_scene: Handle<Scene>,
    target_scene: Handle<Scene>,
}

mod bullet;
mod target;
mod tower;

pub use bullet::*;
pub use target::*;
pub use tower::*;



fn main() {
    App::new()
        // Window Setup
        .insert_resource(ClearColor(Color::rgb(0.2,0.2,0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
                width: WIDTH,
                height: HEIGHT,
                title: "Magic paradise".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..default()
        }))
        // Inspector Setup
        .add_plugin(WorldInspectorPlugin::new())
        // Mod Picking
        .add_plugins(DefaultPickingPlugins)
        // Our Systems
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(BulletPlugin)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_system(camera_controls)
        .run();
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>){
    commands.insert_resource(GameAssets{
        //bullet_scene: assets.load("Bullet.glb#Scene0"),
        //generic_tower_scene: assets.load("GenericTower.glb#Scene0"),
        //tower_scene: assets.load("tower.glb#Scene0"),
        //target_scene: assets.load("Target.glb#Scene0"),

        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        generic_tower_scene: assets.load("TomatoTower.glb#Scene0"),
        tower_scene: assets.load("Tomato.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    });
}

fn camera_controls(
    keyboard:Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
    ){
    let mut camera = camera_query.single_mut();
    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = 3.0;
    let rotate_speed = 0.3;

    //leftwing
    if keyboard.pressed(KeyCode::W){
        camera.translation += forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::S){
        camera.translation -= forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::A){
        camera.translation += left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::D){
        camera.translation -= left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::Q){
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }

    if keyboard.pressed(KeyCode::E){
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,    
){
    //first
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane{size: 25.0})),
        material: materials.add(Color::rgb(0.3,0.5,0.3).into()),
        ..default()
    })
    .insert(Name::new("Ground"));
    
   //second
   //commands.spawn(PbrBundle {
   //     mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
   //     material: materials.add(Color::rgb(0.67,0.84,0.92).into()),
   //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
   //     ..default()
   // })
   // .insert(Tower{
   //     shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
   //     bullet_offset: Vec3::new(0.0, 0.2, 0.5),
   // })
   // .insert(Name::new("Tower"));

    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());


    commands.spawn(SpatialBundle::from_transform(Transform::from_xyz(
            0.0, 0.8, 0.0,
        )))
        .insert(Name::new("Tower_Base"))
        .insert(meshes.add(shape::Capsule::default().into()))
        .insert(Highlighting{
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color),
        })
        .insert(default_collider_color)
        .insert(NotShadowCaster)
        .insert(PickableBundle::default())
        .with_children(|commands|{
            commands.spawn(SceneBundle{
                scene: game_assets.tower_base_scene.clone(),
                transform: Transform::from_xyz(0.0, -0.8, 0.0),
                ..Default::default()
            });
        });

    //third
    commands
        .spawn(SceneBundle{
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-2.0, 0.4, 2.5),

            //mesh: meshes.add(Mesh::from(shape::Cube{ size:0.4})),
            //material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            //transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..Default::default()
        })
        .insert( Target{speed: 0.3})
        .insert( Health{value: 3})
        .insert( Name::new("Target"));
    
    //fourth
    commands
        .spawn(SceneBundle{
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-4.0, 0.4, 2.5),
            //mesh: meshes.add(Mesh::from(shape::Cube{size: 0.4})),
            //material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            //transform: Transform::from_xyz(-4.0, 0.2, 2.5),
            ..Default::default()
        })
        .insert(Target{speed: 0.3})
        .insert(Health{value: 3})
        .insert(Name::new("Target"));
    
    //fifth
    commands.spawn(PointLightBundle {
        point_light: PointLight{
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    })
    .insert(Name::new("Light"));
}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(-2.0,2.5,5.0).looking_at(Vec3::ZERO,Vec3::Y),
        ..default()
    })
    .insert(PickingCameraBundle::default());
}
