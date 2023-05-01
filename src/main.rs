#![allow(dead_code, unused_imports)]
mod bullet;
mod gamedata;
mod player;
mod boss;

use bevy::{
    prelude::*,
    window::{WindowMode, PrimaryWindow},
    utils::Duration,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<gamedata::GameData>()
        .add_startup_system(setup)
        .add_system(player::movement_and_camera)
        .add_system(player::shoot)
        .add_system(boss::aim_at_player)
        .add_system(bullet::handle)
        .add_system(boss::attack_player)
        .run();
}

#[derive(Component)]
pub struct CameraComponent;

fn setup(
    mut commands: Commands, 
    audio: Res<Audio>,
    data: Res<gamedata::GameData>,
    asset_server: Res<AssetServer>,
    window_q: Query<&mut Window, With<PrimaryWindow>>
) {
    let w = window_q.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(w.width()/2.0, w.height()/2.0, 0.0),
            ..default()
        },
        CameraComponent
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ball.png"),
            transform: Transform::from_xyz(data.width/2.0, data.height/2.0, 0.0),
            ..default()
        },
        player::Player::default()
    ));

    let music = asset_server.load("9999.ogg");
    audio.play(music);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("succubus.png"),
            transform: Transform::from_xyz(data.width/3.0, data.height/3.0, 0.0),
            ..default()
        },
        boss::Boss { 
            pattern: boss::AimPattern::Snap,
            bullet_size: 7.5,
            bullet_vel: 175.0,
            timer: Timer::new(
                Duration::from_millis(500), 
                TimerMode::Repeating
            )
        }
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("succubus.png"),
            transform: Transform::from_xyz(
                data.width*0.75, 
                data.height*0.75, 0.0
            ),
            ..default()
        },
        boss::Boss { 
            pattern: boss::AimPattern::Rotate,
            bullet_size: 10.0,
            bullet_vel: 200.0,
            timer: Timer::new(
                Duration::from_millis(333), 
                TimerMode::Repeating
            )
        }
    ));
}
