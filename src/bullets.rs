use crate::animations::{Animation, AnimationState};
use crate::enemies::Enemy;
use crate::player::Player;
use crate::health::Health;
use crate::gamedata::*;
use crate::guns::Gun;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

// ==========
// COMPONENTS
// ==========

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

    #[inline] // means this is used in system function
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

    #[inline]
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


// =======
// SYSTEMS
// =======

// SIMPLIFY BULLETS QUERY
pub fn handle(
    t: Res<Time>,
    data: Res<GameData>,
    mut commands: Commands,
    mut query: ParamSet<(
        Query<(&Transform, &mut Health, &mut AnimationState), With<Player>>, 
        Query<(&Transform, &mut Health, &mut AnimationState), With<Enemy>>
    )>,
    mut bullets: Query<
        (Entity, &mut Transform, &Bullet), 
        (Without<Player>, Without<Enemy>, With<Bullet>)
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
                if let Ok((transform, mut health, mut state)) = query.p0().get_single_mut() {
                    let distance = bt.translation.distance(transform.translation);
                    // in here, the size of the player and boss should be obtained from their
                    // transforms fuck, not the 'gamedata object' 
                    if distance <= bullet.size + transform.scale.x as f32 { // change to HitboxSize, or transform.scale waterfall from the 3d / 2d animations
                        health.0 -= bullet.damage;
                        state.change_if_its_not(Animation::Hurt);
                        commands.entity(bullet_entity).despawn();
                    }
                }
            },
            EntityType::Enemy => {
                for (transform, mut health, mut state) in &mut query.p1() {
                    let distance = bt.translation.distance(transform.translation);
                    if distance <= bullet.size + transform.scale.x * data.scaling as f32 {
                        health.0 -= bullet.damage;
                        state.change_if_its_not(Animation::Hurt);
                        commands.entity(bullet_entity).despawn();
                    }
                }
            }
        }
    }
}
