use crate::boss::Boss;
use crate::player::Player;
use crate::health::Health;
use crate::gamedata::*;
use crate::gun::Gun;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;


#[derive(Component)]
pub struct Bullet {
    pub vel: f32, 
    pub size: f32,
    pub damage: i32,
    pub target: EntityType,
}


impl Bullet {
    pub fn from(gun: &Gun) -> Self {
        Self {
            vel: gun.bullet_vel,
            size: gun.bullet_size,
            damage: gun.bullet_damage,
            target: gun.target
        }
    }

    pub fn spawn(
        g: &Gun, 
        t: &Transform, 
        commands: &mut Commands, 
        meshes: &mut ResMut<Assets<Mesh>>, 
        materials: &mut ResMut<Assets<ColorMaterial>>
    ) {
        commands.spawn((
            Bullet::from(g),
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(g.bullet_size)).into(),
                material: materials.add(ColorMaterial::from(g.color)),
                transform: Transform {
                    translation: t.translation,
                    rotation: g.rotation,
                    ..default()
                },
                ..default()
            }
        ));
    }

    pub fn spawn_straight(
        g: &Gun, 
        t: &Transform, 
        commands: &mut Commands, 
        meshes: &mut ResMut<Assets<Mesh>>, 
        materials: &mut ResMut<Assets<ColorMaterial>>
    ) {
        commands.spawn((
            Bullet::from(g),
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(g.bullet_size)).into(),
                material: materials.add(ColorMaterial::from(g.color)),
                transform: Transform {
                    translation: t.translation,
                    rotation: t.rotation, // only differences, causes bullet to shoot straight from entity tranform
                    ..default()
                },
                ..default()
            }
        ));
    }
}


// SIMPLIFY BULLETS QUERY
pub fn handle(
    t: Res<Time>,
    data: Res<GameData>,
    mut commands: Commands,
    mut query: ParamSet<(
        Query<(&Transform, &mut Health), With<Player>>, 
        Query<(&Transform, &mut Health), With<Boss>>
    )>,
    mut bullets: Query<
        (Entity, &mut Transform, &Bullet), 
        (Without<Player>, Without<Boss>, With<Bullet>)
    >
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
        match bullet.target {
            EntityType::Player => {
                if let Ok((transform, mut health)) = query.p0().get_single_mut() {
                    let distance = bt.translation.distance(transform.translation);
                    // in here, the size of the player and boss should be obtained from their
                    // transforms fuck, not the 'gamedata object' 
                    if distance <= bullet.size + transform.scale.x * 8 as f32 { 
                        health.0 -= bullet.damage;
                        commands.entity(bullet_entity).despawn();
                    }
                }
            },
            EntityType::Enemy => {
                for (transform, mut health) in &mut query.p1() {
                    let distance = bt.translation.distance(transform.translation);
                    if distance <= bullet.size + transform.scale.x * data.scaling as f32 {
                        health.0 -= bullet.damage;
                        commands.entity(bullet_entity).despawn();
                    }
                }
            }
        }
    }
}
