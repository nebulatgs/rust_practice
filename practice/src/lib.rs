use bevy::prelude::*;
use heron::{prelude::*, PhysicsSteps};
use wasm_bindgen::prelude::*;

struct Materials {
    player: Handle<ColorMaterial>,
    bullet: Handle<ColorMaterial>,
    enemy: Handle<ColorMaterial>,
}

struct Enemy;
struct Bullet;

#[wasm_bindgen]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    App::build()
        .insert_resource(ClearColor(Color::hex("212738").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_startup_system(setup.system())
        .add_startup_stage("add_player", SystemStage::single(add_player.system()))
        .add_startup_stage("add_enemy", SystemStage::single(add_enemy.system()))
        .add_system(rotate_enemy.system())
        .add_system(spawn_bullets.system())
        .add_system(move_bullets.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        player: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        bullet: materials.add(Color::rgb(0.2, 0.7, 0.7).into()),
        enemy: materials.add(Color::rgb(0.2, 0.3, 0.7).into()),
    });
}

fn add_player(mut commands: Commands, materials: Res<Materials>) {
    let size = Vec2::new(10.0, 10.0);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player.clone(),
            sprite: Sprite::new(size),
            ..Default::default()
        })
        .insert(RigidBody::KinematicVelocityBased)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            // let the size be consistent with our sprite
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(RotationConstraints::lock());
}

fn add_enemy(mut commands: Commands, materials: Res<Materials>) {
    let size = Vec2::new(50.0, 50.0);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.enemy.clone(),
            sprite: Sprite::new(size),
            transform: Transform::from_xyz(0.0, 300.0, 0.0),
            ..Default::default()
        })
        .insert(Enemy);
}

fn rotate_enemy(mut enemies: Query<(&Enemy, &mut Transform)>) {
    for (_, mut transform) in enemies.iter_mut() {
        transform.rotate(Quat::from_rotation_z(-0.1))
    }
}

fn spawn_bullets(
    mut commands: Commands,
    enemies: Query<(&Enemy, &Transform)>,
    materials: Res<Materials>,
) {
    let size = Vec2::new(10.0, 10.0);
    for (_, transform) in enemies.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.bullet.clone(),
                sprite: Sprite::new(size),
                transform: transform.clone(),
                ..Default::default()
            })
            .insert(RigidBody::Sensor)
            .insert(CollisionShape::Cuboid {
                // let the size be consistent with our sprite
                half_extends: size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .insert(Bullet);
    }
}

fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>) {
    for (_, mut transform) in bullets.iter_mut() {
        let move_dir = transform.rotation * Vec3::Y;
        transform.translation += move_dir;
    }
}
