use crate::bullet::{Bullet, BulletSource};

use bevy::{
    utils::default,
    render::{
        color::Color,
        mesh::{Mesh, shape::Circle}, 
    },
    asset::Assets,
    ecs::{
        component::Component,
        system::{Query, Res, Commands, ResMut},
        query::{With, Without},
    },
    math::Vec3,
    prelude::Camera,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::components::Transform,
    time::{Time, Timer, TimerMode},
    utils::Duration,
    input::{
        Input, 
        keyboard::KeyCode
    }, 
};

pub const SIZE: f32 = 20.0;
pub const SPEED: f32 = 300.0;
pub const ROTATION_SPEED: f32 = 5.0;

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub bullet_size: f32,
    pub bullet_vel: f32,
    pub shooting_timer: Timer,
}

impl Player {
    pub fn default() -> Self {
        Player {
            bullet_size: 2.0,
            bullet_vel: 400.0,
            shooting_timer: Timer::new(
                Duration::from_millis(250), 
                TimerMode::Repeating
            )
        }
    }
}


pub fn movement_and_camera(
    k: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<&mut Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<crate::CameraComponent>)>,
) {
    if let (Ok(mut pt), Ok(mut ct)) = (player.get_single_mut(), camera.get_single_mut()) {
        let mut direction = Vec3::ZERO;
        let mut rotation_factor = 0.0;
        let forward = pt.rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0));
        let right = pt.rotation.mul_vec3(Vec3::new(1.0, 0.0, 0.0));

        if k.pressed(KeyCode::W) { direction += forward; }
        if k.pressed(KeyCode::A) { direction -= right; }
        if k.pressed(KeyCode::S) { direction -= forward; }
        if k.pressed(KeyCode::D) { direction += right; }
        if k.pressed(KeyCode::L) { rotation_factor += 1.0; }
        if k.pressed(KeyCode::Apostrophe) { rotation_factor -= 1.0; }
        if direction.length() > 0.0 { direction = direction.normalize(); }

        let rotation = rotation_factor * ROTATION_SPEED * time.delta_seconds();
        pt.rotate_z(rotation);
        ct.rotate_z(rotation);
        pt.translation += direction * SPEED * time.delta_seconds();
        ct.translation = pt.translation;
    }
}

pub fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player: Query<(&Transform, &mut Player)>,
    k: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    for (pt, mut p) in &mut player {
        if k.just_pressed(KeyCode::P) {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(p.bullet_size).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    transform: Transform {
                        translation: pt.translation,
                        rotation: pt.rotation,
                        ..default()
                    },
                    ..default()
                },
                Bullet { 
                    vel: p.bullet_vel, 
                    size: p.bullet_size,
                    source: BulletSource::Player
                },
            ));      
        } 

        if k.pressed(KeyCode::P) {
            p.shooting_timer.tick(time.delta());
            if p.shooting_timer.finished() {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::new(p.bullet_size).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::GREEN)),
                        transform: Transform {
                            translation: pt.translation,
                            rotation: pt.rotation,
                            ..default()
                        },
                        ..default()
                    },
                    Bullet { 
                        vel: p.bullet_vel,
                        size: p.bullet_size,
                        source: BulletSource::Player
                    },
                ));      
            }
        }
    }
}
