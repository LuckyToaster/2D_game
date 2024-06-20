
use std::{fs::File, io::BufReader};

use crate::enemies::Enemy;
use crate::player::Player;
use crate::gamedata::EntityType;
use crate::bullets::Bullet;
use serde::Deserialize;

use bevy::{
    prelude::*,
    utils::Duration,
};

// =======
// STRUCTS 
// =======

#[derive(Deserialize)]
pub enum QuatType {
    Nan, Identity, Default
}

#[derive(Deserialize)]
pub enum TimerBehavior {
    Once, Repeating
}

#[derive(Deserialize)]
pub struct GunConfig {
    pub pattern: AimPattern,
    pub rotation: QuatType,
    pub timer_duration_millis: u64,
    pub timer_behavior: TimerBehavior,
    pub target: EntityType,
    pub bullet_size: f32,
    pub bullet_vel: f32,
    pub bullet_damage: i32,
    pub bullet_color_r: f32,
    pub bullet_color_g: f32,
    pub bullet_color_b: f32,
}

#[derive(Deserialize)]
pub struct GunConfigs(Vec<GunConfig>);

impl GunConfigs {
    pub fn enemies() -> Vec<GunConfigs> {
        let file = File::open("config/enemies_guns.json").unwrap();
        let reader = BufReader::new(file);
        let data: Vec<GunConfigs> = serde_json::from_reader(reader).unwrap();
        data
    }
}

// ==========
// COMPONENTS
// ==========

#[derive(Component, Deserialize)]
pub enum AimPattern {
    Rotate, Snap, Spiral, PlayerInput 
}

impl AimPattern {       
    //also shoots at player gun if gun is PlayerInput, not great having that there 
    #[inline]           
    fn rotate_gun(
        gun: &mut Gun,
        shooter: &Transform,
        target: &Transform,
        shooter2target: Vec2,
        t: &Res<Time>,
        k: Option<&Res<ButtonInput<KeyCode>>>,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        match gun.pattern {
            AimPattern::Snap => gun.rotation = Quat::from_rotation_arc(Vec3::Y, shooter2target.extend(0.)),
            AimPattern::Rotate => gun.rotation *= Quat::from_rotation_z(AimPattern::get_rotation_angle(shooter2target, *target, t.delta_seconds())), // hm
            AimPattern::Spiral => gun.rotation *= Quat::from_rotation_z(0.20),
            AimPattern::PlayerInput => {
                match k {
                    Some(key) => {
                        if key.just_pressed(KeyCode::KeyP) && gun.timer.finished() {
                            Bullet::spawn_straight(gun, shooter, commands, meshes, materials);   
                        }
                    },
                    None => {}
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
    pub target: EntityType  
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
        target: EntityType
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
    
    pub fn from(gunconfig: GunConfig) -> Self {
        Gun {
            pattern: gunconfig.pattern,
            bullet_size: gunconfig.bullet_size,
            bullet_vel: gunconfig.bullet_vel,
            bullet_damage: gunconfig.bullet_damage,
            color: Color::rgb(
                gunconfig.bullet_color_r as f32, 
                gunconfig.bullet_color_g as f32, 
                gunconfig.bullet_color_b as f32
            ),
            rotation: match gunconfig.rotation {
                QuatType::Default => Quat::default(),
                QuatType::Identity => Quat::IDENTITY,
                QuatType::Nan => Quat::NAN
            },
            target: gunconfig.target,
            timer: Timer::new(
                Duration::from_millis(gunconfig.timer_duration_millis),
                match gunconfig.timer_behavior {
                    TimerBehavior::Once => TimerMode::Once,
                    TimerBehavior::Repeating => TimerMode::Repeating,
                }
            )
        }
    }

    pub fn player_gun() -> Self {
        Gun { 
            pattern: AimPattern::PlayerInput,
            bullet_size: 1.0,
            bullet_vel: 1000.0,
            bullet_damage: 50,
            color: Color::rgb(10.0, 10.0, 10.0),
            rotation: Quat::default(), 
            target: EntityType::Enemy,
            timer: Timer::new(
                Duration::from_millis(200),
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

    pub fn from(gunconfigs: GunConfigs) -> Self {
        let mut guns = Vec::<Gun>::new();
        for gunconfig in gunconfigs.0.into_iter() {
            guns.push(Gun::from(gunconfig));
        } 
        Self(guns)
    }
}


// =======
// SYSTEMS
// =======

pub fn enemy_guns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut enemies_q: Query<(&Transform, &mut Guns), With<Enemy>>,
    player_q: Query<&Transform, With<Player>>,
    t: Res<Time>,
) {
    for (et, mut guns) in enemies_q.iter_mut() {
        if let Ok(pt) = player_q.get_single() {
            let e2p = (pt.translation.truncate() - et.translation.truncate()).normalize();

            for gun in guns.0.iter_mut() {
                AimPattern::rotate_gun(gun, et, pt, e2p, &t, None, &mut commands, &mut meshes, &mut materials);

                gun.timer.tick(t.delta());
                if gun.timer.just_finished() { 
                    Bullet::spawn(gun, et, &mut commands, &mut meshes, &mut materials); 
                }
            }
        } else {
            continue; // the 'smart' guns that aim at the player shouldn't fire
        }
    }
}


pub fn player_guns(
    t: Res<Time>,
    k: Res<ButtonInput<KeyCode>>,
    enemy_q: Query<&Transform, With<Enemy>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut players_q: Query<(&Transform, &mut Guns), With<Player>>,
) {
    for (pt, mut guns) in players_q.iter_mut() {
        for et in enemy_q.iter() {
            let p2e = (et.translation.truncate() - pt.translation.truncate()).normalize();

            for gun in guns.0.iter_mut() {
                AimPattern::rotate_gun(gun, pt, et, p2e, &t, Some(&k), &mut commands, &mut meshes, &mut materials);

                gun.timer.tick(t.delta());
                if gun.timer.just_finished() {
                    Bullet::spawn(gun, pt, &mut commands, &mut meshes, &mut materials);
                }
            }
        }
    }
}

