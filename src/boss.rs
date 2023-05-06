#![allow(unused_imports)]
use crate::bullets::{Bullet, BulletSource};
use crate::player::Player;
use crate::health::Health;
use bevy::{
    prelude::*,
    utils::Duration,
    render::{mesh::{shape::Circle, Mesh}, color::Color},
    sprite::{SpriteBundle, MaterialMesh2dBundle, ColorMaterial},
};

#[derive(Component)]
pub enum AimPattern {
    Rotate, Snap, Spiral,
}

#[derive(Component)]
pub struct Gun {
    pub pattern: AimPattern,
    pub bullet_size: f32,
    pub bullet_vel: f32,
    pub color: Color,
    pub rotation: Quat,
    pub timer: Timer
}

#[derive(Component)]
pub struct Boss {
    pub guns: Vec<Gun>,
}


pub fn spawn(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("Tilemap/tilemap.png"),
        Vec2::new(16.0, 16.0), 
        12, 
        11, 
        Some(Vec2::new(1.0, 1.0)), 
        None
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        crate::player::AnimationIndices { first: 84, last: 88 },
        crate::player::AnimationTimer(
            Timer::from_seconds(
                0.1, 
                TimerMode::Repeating
            )
        ),
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(84), 
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        },
        Health(100),
        Boss {
            guns: vec![
                Gun { 
                    pattern: AimPattern::Snap,
                    bullet_size: 5.0,
                    bullet_vel: 200.0,
                    color: Color::rgb(5.5, 1.0, 9.5),
                    rotation: Quat::NAN,
                    timer: Timer::new(
                        Duration::from_millis(200), 
                        TimerMode::Repeating
                    )
                },
                Gun { 
                    pattern: AimPattern::Spiral,
                    bullet_size: 8.0,
                    bullet_vel: 300.0,
                    color: Color::rgb(7.5, 0.0, 7.5),
                    rotation: Quat::IDENTITY,
                    timer: Timer::new(
                        Duration::from_millis(50), 
                        TimerMode::Repeating
                    )
                },
                Gun { 
                    pattern: AimPattern::Rotate,
                    bullet_size: 20.0,
                    bullet_vel: 150.0,
                    color: Color::rgb(1.0, 0.75, 5.5),
                    rotation: Quat::default(),
                    timer: Timer::new(
                        Duration::from_millis(150), 
                        TimerMode::Repeating
                    )
                },
            ]
        }
    ));
}


pub fn aim_at_player(
    mut boss_q: Query<(&mut Transform, &mut Boss), Without<Player>>,
    player_q: Query<&Transform, With<Player>>,
    t: Res<Time>
) {
    //let pt = player_q.single().translation.truncate();

    if let Ok(transform) = player_q.get_single() {
        let pt = transform.translation.truncate();
        for (bt, mut boss) in &mut boss_q {
            let b2p: Vec2 = (pt - bt.translation.truncate()).normalize();
            let bt_cp = bt.clone();
            for gun in boss.guns.iter_mut() {
                match gun.pattern {
                    AimPattern::Snap => gun.rotation = Quat::from_rotation_arc(Vec3::Y, b2p.extend(0.)),
                    AimPattern::Rotate => gun.rotation *= Quat::from_rotation_z(get_rotation_angle(b2p, bt_cp, t.delta_seconds())),
                    AimPattern::Spiral => gun.rotation *= Quat::from_rotation_z(0.20)
                }
            }
        }
    }
}


pub fn shoot_player(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut boss_q: Query<(&Transform, &mut Boss)>,
    time: Res<Time>
) {
    for (bt, mut boss) in &mut boss_q {
        for gun in boss.guns.iter_mut() {
            gun.timer.tick(time.delta());
            if gun.timer.just_finished() {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::new(gun.bullet_size).into()).into(),
                        material: materials.add(ColorMaterial::from(gun.color)),
                        transform: Transform {
                            translation: bt.translation,
                            rotation: gun.rotation,
                            ..default()
                        },
                        ..default()
                    },
                    Bullet { 
                        vel: gun.bullet_vel, 
                        size: gun.bullet_size,
                        damage: 10,
                        source: BulletSource::Enemy
                    },
                ));      
            }
        }
    }
}


#[inline]
fn get_rotation_angle(b2p: Vec2, bt: Transform, t: f32) -> f32 {
    let b_forward = (bt.rotation * Vec3::Y).truncate();
    let forward_dot_player = b_forward.dot(b2p);
    if (forward_dot_player - 1.0).abs() < f32::EPSILON { return 0.0;}
    let b_right = (bt.rotation * Vec3::X).truncate();
    let right_dot_player = b_right.dot(b2p);
    let rotation_sign = -f32::copysign(1.0, right_dot_player);
    let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos(); 
    return rotation_sign * (f32::to_radians(90.0) * t).min(max_angle);
}

