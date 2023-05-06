use crate::boss::Boss;
use crate::player::Player;
use crate::health::Health;
use crate::gamedata::GameData;
use bevy::prelude::*;

pub enum BulletSource {
    Enemy, Player
}

#[derive(Component)]
pub struct Bullet {
    pub vel: f32, 
    pub size: f32,
    pub damage: i32,
    pub source: BulletSource,
}


pub fn handle(
    t: Res<Time>,
    data: Res<GameData>,
    mut commands: Commands,
    mut boss_query: Query<(&Transform, &mut Health), With<Boss>>,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    mut bullets: Query<(Entity, &mut Transform, &Bullet), (Without<Player>, Without<Boss>)>
) {

    for (bullet_entity, mut bt, bullet) in &mut bullets {
        // update
        let base_direction = Vec3::new(0.0, 1.0, 0.0); 
        let rotated_direction = bt.rotation.mul_vec3(base_direction);
        let displacement = rotated_direction * bullet.vel * t.delta_seconds();
        bt.translation.x += displacement.x;
        bt.translation.y += displacement.y;

        // drop when outside_bounds
        let outside_width_bounds = bt.translation.x >= data.width || bt.translation.x <= -data.width;
        let outside_height_bounds = bt.translation.y >= data.height || bt.translation.y <= -data.height;

        if outside_width_bounds || outside_height_bounds {
            commands.entity(bullet_entity).despawn();
        }

        // collision detection
        match bullet.source {
            BulletSource::Enemy => {
                if let Ok((transform, mut health)) = player_query.get_single_mut() {
                    let distance = bt.translation.distance(transform.translation);
                    if distance <= bullet.size + 10.0 { 
                        health.0 -= bullet.damage;
                    }
                }
            },
            BulletSource::Player => {
                for (transform, mut health) in &mut boss_query {
                    let distance = bt.translation.distance(transform.translation);
                    if distance <= bullet.size + 10.0 {
                        health.0 -= bullet.damage;
                    }
                }
            }
        }
    }
}

/*pub fn handle(
    t: Res<Time>,
    data: Res<GameData>,
    mut commands: Commands,
    enemy: Query<(Entity, &Transform), With<Boss>>,
    player: Query<(Entity, &Transform), With<Player>>,
    mut bullets: Query<(Entity, &mut Transform, &Bullet), (Without<Player>, Without<Boss>)>
) {
    for (bullet_entity, mut bt, b) in &mut bullets {
        // update
        let base_direction = Vec3::new(0.0, 1.0, 0.0); 
        let rotated_direction = bt.rotation.mul_vec3(base_direction);
        let displacement = rotated_direction * b.vel * t.delta_seconds();
        bt.translation.x += displacement.x;
        bt.translation.y += displacement.y;

        // drop when outside_bounds
        let outside_width_bounds = bt.translation.x >= data.width || bt.translation.x <= -data.width;
        let outside_height_bounds = bt.translation.y >= data.height || bt.translation.y <= -data.height;

        if outside_width_bounds || outside_height_bounds {
            commands.entity(bullet_entity).despawn();
        }

        // collision detection
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
}*/
