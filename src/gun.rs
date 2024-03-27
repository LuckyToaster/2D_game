use crate::boss::Boss;
use crate::player::Player;
use crate::gamedata::*;
use crate::bullets::Bullet;

use bevy::{
    prelude::*,
    utils::Duration,
};


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
    pub target: EntityType  // should this be removed
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

    // move this kind of stuff to config file
    pub fn player_gun() -> Self {
        Gun { 
            pattern: AimPattern::PlayerInput,
            bullet_size: 2.0,
            bullet_vel: 450.0,
            bullet_damage: 5,
            color: Color::rgb(0.0, 0.0, 50.0),
            rotation: Quat::default(), // make sense of this with player
            target: EntityType::Enemy,
            timer: Timer::new(
                Duration::from_millis(100),
                TimerMode::Once
            )
        }
    }

    pub fn default_spiral(target: EntityType) -> Self {
        Gun { 
            pattern: AimPattern::Spiral,
            bullet_size: 8.0,
            bullet_vel: 275.0,
            bullet_damage: 15,
            color: Color::rgb(7.5, 0.0, 7.5),
            rotation: Quat::IDENTITY,
            target: target,
            timer: Timer::new(
                Duration::from_millis(50), 
                TimerMode::Repeating
            )
        }
    }

    pub fn default_snap(target: EntityType) -> Self {
        Gun { 
            pattern: AimPattern::Snap,
            bullet_size: 5.0,
            bullet_vel: 175.0,
            bullet_damage: 2,
            color: Color::rgb(5.5, 1.0, 9.5),
            rotation: Quat::NAN,
            target: target,
            timer: Timer::new(
                Duration::from_millis(200), 
                TimerMode::Repeating
            )
        }
    }

    pub fn default_rotate(target: EntityType) -> Self {
        Gun { 
            pattern: AimPattern::Rotate,
            bullet_size: 15.0,
            bullet_vel: 112.5,
            bullet_damage: 5,
            color: Color::rgb(1.0, 0.75, 5.5),
            rotation: Quat::default(),
            target: target,
            timer: Timer::new(
                Duration::from_millis(150), 
                TimerMode::Repeating
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


pub fn enemy_guns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut enemies_q: Query<(&Transform, &mut Guns), With<Boss>>,
    player_q: Query<&Transform, With<Player>>,
    t: Res<Time>,
) {
    for (et, mut guns) in enemies_q.iter_mut() {
        let pt = player_q.get_single().unwrap();
        let e2p = (pt.translation.truncate() - et.translation.truncate()).normalize();

        for gun in guns.0.iter_mut() {
            gun.timer.tick(t.delta());

            match gun.pattern {
                AimPattern::Snap => gun.rotation = Quat::from_rotation_arc(Vec3::Y, e2p.extend(0.)),
                AimPattern::Rotate => gun.rotation *= Quat::from_rotation_z(get_rotation_angle(e2p, *pt, t.delta_seconds())), 
                AimPattern::Spiral => gun.rotation *= Quat::from_rotation_z(0.20),
                AimPattern::PlayerInput => todo!("fugg")
            }

            if gun.timer.just_finished() { 
                Bullet::spawn(gun, et, &mut commands, &mut meshes, &mut materials); 
            }
        }
    }
}


pub fn player_guns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut players_q: Query<(&Transform, &mut Guns), With<Player>>,
    enemy_q: Query<&Transform, With<Boss>>,
    t: Res<Time>,
    k: Res<ButtonInput<KeyCode>>,
) {

    for (pt, mut guns) in players_q.iter_mut() {
        let et = enemy_q.get_single().unwrap();
        let p2e = (et.translation.truncate() - pt.translation.truncate()).normalize();

        for gun in guns.0.iter_mut() {
            gun.timer.tick(t.delta());

            match gun.pattern {
                AimPattern::Snap => gun.rotation = Quat::from_rotation_arc(Vec3::Y, p2e.extend(0.)),
                AimPattern::Rotate => gun.rotation *= Quat::from_rotation_z(get_rotation_angle(p2e, *et, t.delta_seconds())), // hm
                AimPattern::Spiral => gun.rotation *= Quat::from_rotation_z(0.20),
                AimPattern::PlayerInput => {
                    if k.just_pressed(KeyCode::KeyP) && gun.timer.finished() {
                        Bullet::spawn_straight(gun, pt, &mut commands, &mut meshes, &mut materials);   
                    }
                } 
            }

            if gun.timer.just_finished() {
                Bullet::spawn(gun, pt, &mut commands, &mut meshes, &mut materials);
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
