use bevy::prelude::*;

// use this to shoot bullets with a source
#[derive(Component)]
pub enum Target {
    Player, Enemy 
}

#[derive(Component)]
pub enum AimPattern {
    Rotate, Snap, Spiral, PlayerInput 
}

#[derive(Component)]
pub struct Gun {
    pub pattern: AimPattern,
    pub bullet_size: f32,
    pub bullet_vel: f32,
    pub bullet_damage: i32,
    pub color: Color,
    pub rotation: Quat,
    pub timer: Timer,
    pub target: Target  // should this be removed
}


use crate::boss::Boss;
use crate::player::Player;
use crate::bullets::{Bullet, BulletSource};
use bevy::sprite::MaterialMesh2dBundle;

// aim_and_shoot all guns
pub fn aim_and_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gun_q: Query<&mut Gun>,
    player_q: Query<(&Transform, &Player)>,
    boss_q: Query<&Transform, With<Boss>>,  
    k: Res<ButtonInput<KeyCode>>,
    t: Res<Time>,

    //player_q2: Query<&Transform, With<Player>>,
    //mut boss_q2: Query<(&Transform, &mut Boss)>,
) {
    if let (Ok((player_t, player)), Ok(boss_t)) = (player_q.get_single(), boss_q.get_single()) {
        for mut gun in gun_q.iter_mut() {
            // aim guns and player shoot
            let b2p: Vec2 = (player_t.translation.truncate() - boss_t.translation.truncate()).normalize();
            match gun.pattern {
                AimPattern::Snap => gun.rotation = Quat::from_rotation_arc(Vec3::Y, b2p.extend(0.)),
                AimPattern::Rotate => gun.rotation *= Quat::from_rotation_z(get_rotation_angle(b2p, *boss_t, t.delta_seconds())),
                AimPattern::Spiral => gun.rotation *= Quat::from_rotation_z(0.20),
                AimPattern::PlayerInput =>  {   // player shoots through the aimpattern (player input)
                    if k.just_pressed(KeyCode::KeyP) {
                        commands.spawn((
                            MaterialMesh2dBundle {
                                mesh: meshes.add(Circle::new(player.bullet_size)).into(),
                                material: materials.add(ColorMaterial::from(Color::rgb(6.25, 9.4, 9.1))),
                                transform: Transform {
                                    translation: player_t.translation,
                                    rotation: player_t.rotation,
                                    ..default()
                                },
                                ..default()
                            },
                            Bullet { 
                                vel: player.bullet_vel, 
                                size: player.bullet_size,
                                damage: 10,
                                source: BulletSource::Player
                            }
                        ));      
                    } 
                }
            }
            // boss shoot guns    
            match gun.target {
                Target::Enemy => println!("helo lol"),
                Target::Player => {
                    gun.timer.tick(t.delta());
                    if gun.timer.just_finished() {
                        commands.spawn((
                            MaterialMesh2dBundle {
                                mesh: meshes.add(Circle::new(gun.bullet_size)).into(),
                                material: materials.add(ColorMaterial::from(gun.color)),
                                transform: Transform {
                                    translation: boss_t.translation,
                                    rotation: gun.rotation,
                                    ..default()
                                },
                                ..default()
                            },
                            Bullet { 
                                vel: gun.bullet_vel, 
                                size: gun.bullet_size,
                                damage: gun.bullet_damage,
                                source: BulletSource::Enemy
                            },
                        ));      
                    }
                }
            }
        } 
    }
    
    /* 
    // boss::aim
    if let Ok(transform) = player_q2.get_single() {
        let pt = transform.translation.truncate();
        for (bt, mut boss) in &mut boss_q2 {
            let b2p: Vec2 = (pt - bt.translation.truncate()).normalize();
            let bt_cp = bt.clone();
            for gun in boss.guns.iter_mut() {
                match gun.pattern {
                    AimPattern::Snap => gun.rotation = Quat::from_rotation_arc(Vec3::Y, b2p.extend(0.)),
                    AimPattern::Rotate => gun.rotation *= Quat::from_rotation_z(get_rotation_angle(b2p, bt_cp, t.delta_seconds())),
                    AimPattern::Spiral => gun.rotation *= Quat::from_rotation_z(0.20),
                    _ => println!("fugg!")
                }
            }
        }
    }


    // boss::shoot
    for (bt, mut boss) in &mut boss_q2 {
        for gun in boss.guns.iter_mut() {
            gun.timer.tick(t.delta());
            if gun.timer.just_finished() {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::new(gun.bullet_size)).into(),
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
                        damage: gun.bullet_damage,
                        source: BulletSource::Enemy
                    },
                ));      
            }
        }
    }

    // player::shoot
    for (pt, p) in &player_q {
        if k.just_pressed(KeyCode::KeyP) {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(p.bullet_size)).into(),
                    material: materials.add(ColorMaterial::from(Color::rgb(6.25, 9.4, 9.1))),
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
                    damage: 10,
                    source: crate::bullets::BulletSource::Player
                },
            ));      
        } 
    }
    */
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

/* 
pub fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player: Query<(&Transform, &Player)>,
    k: Res<ButtonInput<KeyCode>>,
) {
    for (pt, p) in &player {
        if k.just_pressed(KeyCode::KeyP) {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(p.bullet_size)).into(),
                    material: materials.add(ColorMaterial::from(Color::rgb(6.25, 9.4, 9.1))),
                    transform: Transform {
                        translation: pt.translation,
                        rotation: pt.rotation,
                        ..default()
                    },
                    ..default()
                },
                crate::bullets::Bullet { 
                    vel: p.bullet_vel, 
                    size: p.bullet_size,
                    damage: 10,
                    source: crate::bullets::BulletSource::Player
                },
            ));      
        } 
    }
}

// put this in gun file, generic guns should aim at their targets?
pub fn aim_at_player(
    mut boss_q: Query<(&mut Transform, &mut Boss), Without<Player>>,
    player_q: Query<&Transform, With<Player>>,
    t: Res<Time>
) {
    if let Ok(transform) = player_q.get_single() {
        let pt = transform.translation.truncate();
        for (bt, mut boss) in &mut boss_q {
            let b2p: Vec2 = (pt - bt.translation.truncate()).normalize();
            let bt_cp = bt.clone();
            for gun in boss.guns.iter_mut() {
                match gun.pattern {
                    AimPattern::Snap => gun.rotation = Quat::from_rotation_arc(Vec3::Y, b2p.extend(0.)),
                    AimPattern::Rotate => gun.rotation *= Quat::from_rotation_z(get_rotation_angle(b2p, bt_cp, t.delta_seconds())),
                    AimPattern::Spiral => gun.rotation *= Quat::from_rotation_z(0.20),
                    _ => println!("fugg!")
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
                        mesh: meshes.add(Circle::new(gun.bullet_size)).into(),
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
                        damage: gun.bullet_damage,
                        source: BulletSource::Enemy
                    },
                ));      
            }
        }
    }
}

*/
