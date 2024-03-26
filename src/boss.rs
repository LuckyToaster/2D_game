#![allow(unused_imports)]
use crate::bullets::{Bullet, BulletSource};
use crate::player::Player;
use crate::health::Health;
use crate::gamedata::GameData;
use crate::gun::{Gun, Guns, Target, AimPattern};

use bevy::sprite::Mesh2dHandle;
use bevy::{animation, text};
use bevy::{
    prelude::*,
    utils::Duration,
    render::{mesh::{/*shape::Circle,*/ Mesh}, color::Color},
    sprite::{SpriteBundle, MaterialMesh2dBundle, ColorMaterial},
};

use bevy_math::primitives::Circle;


#[derive(Component)]
pub struct Boss {
    //pub guns: Vec<Gun>,
    pub size: f32,
}


pub fn spawn(
    data: Res<GameData>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Tilemap/tilemap.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 12, 11, Some(Vec2::new(1.0, 1.0)), Some(Vec2::new(0.0, 0.0)));
    let texture_atlas_layout = texture_atlases.add(layout);
    let animation_indices = crate::player::AnimationIndices {first: 84, last: 88};

    commands.spawn((
        Health(100),
        //Target::Boss,
        animation_indices,
        crate::player::AnimationTimer(
            Timer::from_seconds(
                0.1, 
                TimerMode::Repeating
            )
        ),
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout, 
                index: 84 // cannot do animation indices.first because move, cannot do &animation_indices.first
            },
            transform: Transform {
                translation: Vec3::new(100.0, 100.0, 0.0), 
                rotation: Quat::default(), 
                scale: Vec3::splat(3.0),
            },
            ..default()
        },
        Boss {
            size: (8 * data.scaling) as f32,
        },
        Guns::new(
            vec![
                Gun { 
                    pattern: AimPattern::Spiral,
                    bullet_size: 8.0,
                    bullet_vel: 275.0,
                    bullet_damage: 15,
                    color: Color::rgb(7.5, 0.0, 7.5),
                    rotation: Quat::IDENTITY,
                    target: Target::Player,
                    timer: Timer::new(
                        Duration::from_millis(50), 
                        TimerMode::Repeating
                    )
                },
                Gun { 
                    pattern: AimPattern::Snap,
                    bullet_size: 5.0,
                    bullet_vel: 175.0,
                    bullet_damage: 10,
                    color: Color::rgb(5.5, 1.0, 9.5),
                    rotation: Quat::NAN,
                    target: Target::Player,
                    timer: Timer::new(
                        Duration::from_millis(200), 
                        TimerMode::Repeating
                    )
                },
                Gun { 
                    pattern: AimPattern::Rotate,
                    bullet_size: 15.0,
                    bullet_vel: 112.5,
                    bullet_damage: 5,
                    color: Color::rgb(1.0, 0.75, 5.5),
                    rotation: Quat::default(),
                    target: Target::Player,
                    timer: Timer::new(
                        Duration::from_millis(150), 
                        TimerMode::Repeating
                    )
                },
            ]
        )
    ));
}

