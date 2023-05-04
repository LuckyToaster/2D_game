use crate::gamedata::GameData;
use crate::boss::Boss;
use crate::player::Player;
use bevy::prelude::*;

pub enum BulletSource {
    Enemy, Player
}

#[derive(Component)]
pub struct Bullet {
    pub vel: f32, 
    pub size: f32,
    pub source: BulletSource,
}


pub fn handle(
    t: Res<Time>,
    data: Res<GameData>,
    mut commands: Commands,
    enemy: Query<(Entity, &Transform), With<Boss>>,
    player: Query<(Entity, &Transform), With<Player>>,
    mut bullets: Query<(Entity, &mut Transform, &Bullet), (Without<Player>, Without<Boss>)>
) {
    for (ee, mut bt, b) in &mut bullets {
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
            commands.entity(ee).despawn();
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
}
