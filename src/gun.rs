use crate::boss::Boss;
use crate::player::Player;
use crate::bullets::{Bullet, BulletSource};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{
    prelude::*,
    utils::Duration,
};

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

impl Gun {
    pub fn new(
        pattern: AimPattern, 
        bullet_size: f32, 
        bullet_vel: f32, 
        bullet_damage: i32, 
        color: Color, 
        rotation: Quat, 
        timer: Timer, 
        target: Target
    ) -> Self {
        Gun { 
            pattern, 
            bullet_size, 
            bullet_vel, 
            bullet_damage, 
            color, 
            rotation, 
            timer, 
            target 
        }
    }

    // move this kind of stuff to config file
    pub fn player_gun() -> Self {
        Gun { 
            pattern: AimPattern::PlayerInput,
            bullet_size: 2.0,
            bullet_vel: 450.0,
            bullet_damage: 5,
            color: Color::rgb(6.25, 9.4, 9.1),
            rotation: Quat::default(), // make sense of this with player
            target: Target::Enemy,
            timer: Timer::new(
                Duration::from_millis(100),
                TimerMode::Once
            )
        }
    }
}


#[derive(Component)]
pub struct Guns(Vec<Gun>);

impl Guns {
    pub fn new(guns: Vec<Gun>) -> Self {
        Self(guns)
    }
}

// aim_and_shoot all guns
pub fn aim_and_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shooter_q: Query<(&Transform, &mut Guns)>,
    player_q: Query<&Transform, With<Player>>,
    boss_q: Query<&Transform, With<Boss>>,
    k: Res<ButtonInput<KeyCode>>,
    t: Res<Time>,
) {

    let player_t = player_q.get_single().unwrap();
    let boss_t = boss_q.get_single().unwrap();

    for (shooter_t, mut guns) in shooter_q.iter_mut() {     
        for gun in guns.0.iter_mut() {
            gun.timer.tick(t.delta());
            match gun.target {
                Target::Enemy => {
                    let shooter2boss = (boss_t.translation.truncate() - shooter_t.translation.truncate()).normalize();
                    match gun.pattern {
                        AimPattern::Snap => gun.rotation = Quat::from_rotation_arc(Vec3::Y, shooter2boss.extend(0.)),
                        AimPattern::Rotate => gun.rotation *= Quat::from_rotation_z(get_rotation_angle(shooter2boss, *boss_t, t.delta_seconds())), // hm
                        AimPattern::Spiral => gun.rotation *= Quat::from_rotation_z(0.20),
                        AimPattern::PlayerInput => {
                            if k.just_pressed(KeyCode::KeyP)  && gun.timer.finished() { // fix cooldown for shooting
                                commands.spawn((
                                    MaterialMesh2dBundle {
                                        mesh: meshes.add(Circle::new(gun.bullet_size)).into(),
                                        material: materials.add(ColorMaterial::from(Color::rgb(6.25, 9.4, 9.1))),
                                        transform: Transform {
                                            translation: shooter_t.translation,
                                            rotation: shooter_t.rotation,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Bullet { 
                                        vel: gun.bullet_vel, 
                                        size: gun.bullet_size,
                                        damage: gun.bullet_damage,
                                        source: BulletSource::Player
                                    }
                                ));      
                            } 
                        }
                    }
                }, 
                Target::Player => {
                    let shooter2player = (player_t.translation.truncate() - shooter_t.translation.truncate()).normalize();
                    match gun.pattern {
                        AimPattern::Snap => gun.rotation = Quat::from_rotation_arc(Vec3::Y, shooter2player.extend(0.)),
                        AimPattern::Rotate => gun.rotation *= Quat::from_rotation_z(get_rotation_angle(shooter2player, *shooter_t, t.delta_seconds())),
                        AimPattern::Spiral => gun.rotation *= Quat::from_rotation_z(0.20),
                        AimPattern::PlayerInput => todo!(), // hmm
                    }

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
