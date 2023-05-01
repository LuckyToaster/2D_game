use crate::player::Player;
use crate::gamedata::GameData;
use crate::bullet::{Bullet, BulletSource};
use bevy::{
    utils::default,
    transform::components::Transform,
    render::{mesh::Mesh, color::Color},
    prelude::shape::Circle,
    math::{Vec3, Vec2, Quat},
    time::{Time, Timer, TimerMode},
    sprite::{MaterialMesh2dBundle, ColorMaterial, Material2d},
    asset::{Assets, Handle},
    utils::Duration,
    ecs::{
        component::Component,
        system::{Query, Res, ResMut, Commands},
        query::{With, Without},
    },
};

#[derive(Component)]
pub enum AimPattern {
    Rotate, Snap
}

#[derive(Component)]
pub struct Boss {
    pub pattern: AimPattern,
    pub bullet_size: f32,
    pub bullet_vel: f32,
    pub timer: Timer,
}


pub fn aim_at_player(
    mut boss_q: Query<(&mut Transform, &Boss), Without<Player>>,
    player_q: Query<&Transform, With<Player>>,
    time: Res<Time>
) {
    let pt = player_q.single().translation.truncate();
    for (mut bt, boss) in &mut boss_q {
        let b2p = (pt - bt.translation.truncate()).normalize();
        match boss.pattern {
            AimPattern::Snap => bt.rotation = Quat::from_rotation_arc(Vec3::Y, b2p.extend(0.)),
            AimPattern::Rotate => {
                let b_forward = (bt.rotation * Vec3::Y).truncate();
                let forward_dot_player = b_forward.dot(b2p);
                if (forward_dot_player - 1.0).abs() < f32::EPSILON { continue; }
                let b_right = (bt.rotation * Vec3::X).truncate();
                let right_dot_player = b_right.dot(b2p);
                let rotation_sign = -f32::copysign(1.0, right_dot_player);
                let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos(); 
                let rotation_angle = rotation_sign * (f32::to_radians(90.0) 
                    * time.delta_seconds()).min(max_angle);
                bt.rotate_z(rotation_angle);
            }
        }
    }
}

pub fn attack_player(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut boss_q: Query<(&Transform, &mut Boss)>,
    time: Res<Time>
) {
    for (bt, mut b) in &mut boss_q {
        b.timer.tick(time.delta());
        if  b.timer.just_finished() {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(b.bullet_size).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    transform: Transform {
                        translation: bt.translation,
                        rotation: bt.rotation,
                        ..default()
                    },
                    ..default()
                },
                Bullet { 
                    vel: b.bullet_vel, 
                    size: b.bullet_size,
                    source: BulletSource::Enemy
                },
            ));      
        }
    }
}



