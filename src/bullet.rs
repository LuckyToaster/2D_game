//#![allow(dead_code)]
use crate::gamedata::GameData;
use crate::boss::Boss;
use crate::player::Player;

use bevy::{
    transform::components::Transform,
    render::color::Color,
    time::Time,
    math::{Vec2, Vec3},
    ecs::{
        entity::Entity,
        component::Component,
        query::{With, Without},
        system::{Query, Res, Commands},
    },
};

pub enum BulletSource {
    Enemy, Player
}

#[derive(Component)]
pub struct Bullet {
    pub vel: f32, 
    pub size: f32,
    pub source: BulletSource,
}


/*pub fn update_bullets(
    mut bullets: Query<(&mut Transform, &Bullet)>, 
    time: Res<Time>
) {
    for (mut transform, b) in &mut bullets {
        let base_direction = Vec3::new(0.0, 1.0, 0.0); 
        let rotated_direction = transform.rotation.mul_vec3(base_direction);
        let displacement = rotated_direction * b.vel * time.delta_seconds();
        transform.translation.x += displacement.x;
        transform.translation.y += displacement.y;
    }
}*/

pub fn handle(
    mut commands: Commands,
    player: Query<(Entity, &Transform), With<Player>>,
    enemy: Query<(Entity, &Transform), With<Boss>>,
    mut bullets: Query<(Entity, &mut Transform, &Bullet), (Without<Player>, Without<Boss>)>, // shitty query, cant do a 'With<Bullet>' for some reason 
    data: Res<GameData>,
    t: Res<Time>
) {
    for (ee, mut bt, b) in &mut bullets {
        let base_direction = Vec3::new(0.0, 1.0, 0.0); 
        let rotated_direction = bt.rotation.mul_vec3(base_direction);
        let displacement = rotated_direction * b.vel * t.delta_seconds();
        bt.translation.x += displacement.x;
        bt.translation.y += displacement.y;

        let x = bt.translation.x;
        let y = bt.translation.y;

        if x >= data.width || x <= 0.0 || y >= data.height || y <= 0.0 {
            commands.entity(ee).despawn();
        }
        
        match b.source {
            BulletSource::Enemy => {
                if let Some((pe, pt)) = player.iter().next() {
                    let distance = bt.translation.distance(pt.translation);
                    if distance <= b.size + 10.0 { 
                        commands.entity(pe).despawn(); 
                    }
                }
            },

            BulletSource::Player => {
                if let Some((ee, et)) = enemy.iter().next() {
                    let distance = bt.translation.distance(et.translation);
                    if distance <= b.size + 10.0 {
                        commands.entity(ee).despawn();
                    }
                }
            }
        }
    }
}
