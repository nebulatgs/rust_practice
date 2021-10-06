use bevy::prelude::*;
use heron::{prelude::*, PhysicsSteps};
use wasm_bindgen::prelude::*;

struct Materials {
    block: Handle<ColorMaterial>,
    ground: Handle<ColorMaterial>,
}

#[wasm_bindgen]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    App::build()
        .insert_resource(ClearColor(Color::hex("212738").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .insert_resource(Gravity::from(Vec3::new(0.0, -300.0, 0.0)))
        // .insert_resource(PhysicsSteps::from(1.0))
        .add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_block.system()))
        .add_startup_stage("add_ground", SystemStage::single(spawn_ground.system()))
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        block: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        ground: materials.add(Color::rgb(0.2, 0.7, 0.7).into()),
    });
}

fn spawn_block(mut commands: Commands, materials: Res<Materials>) {
    let size = Vec2::new(50.0, 50.0);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.block.clone(),
            sprite: Sprite::new(size),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            // let the size be consistent with our sprite
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(RotationConstraints::lock());
}

fn spawn_ground(mut commands: Commands, materials: Res<Materials>) {
    let size = Vec2::new(500.0, 50.0);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.ground.clone(),
            sprite: Sprite::new(size),
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            // let the size be consistent with our sprite
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(RotationConstraints::lock());
}
