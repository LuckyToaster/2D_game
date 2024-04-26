use crate::health::Health;
use crate::gamedata::*;
use crate::guns::{ GunConfigs, Guns };
use crate::animations::{AnimationIndices, AnimationTimer, SpriteSheetConfig};
use bevy::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use itertools::izip;

// =======
// STRUCTS
// =======

#[derive(Component)]
pub struct Enemy;

#[derive(Deserialize)]
pub struct EnemyConfig {
    pub pos_x: f32,
    pub pos_y: f32,
    pub health: i32,
}

impl EnemyConfig {
    pub fn enemies() -> Vec<EnemyConfig> {
        let file = File::open("config/enemies.json").unwrap();
        let reader = BufReader::new(file);
        let enemies: Vec<EnemyConfig> = serde_json::from_reader(reader).unwrap();
        enemies
    }
}


// =======
// SYSTEMS
// =======

pub fn spawn(
    gamedata: Res<GameData>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let gunconfigs_vec = GunConfigs::enemies();
    let sheets = SpriteSheetConfig::enemies();
    let enemies = EnemyConfig::enemies();

    for (gunconfigs, sheet, enemy) in izip!(gunconfigs_vec, sheets, enemies) {
        let guns = Guns::from(gunconfigs);
        let mut t = Transform::from_scale(Vec3::splat(gamedata.player_size));
        t.translation = Vec3::new(enemy.pos_x, enemy.pos_y, 0.0);

        commands.spawn((
            Enemy,
            Health(100),
            guns,
            AnimationIndices { first: sheet.first_animation_index, last: sheet.last_animation_index }, // from here to the end is the animation data
            AnimationTimer(Timer::from_seconds(sheet.duration_s, TimerMode::Repeating)),
            SpriteSheetBundle {
                transform: t,
                //Transform::from_scale(Vec3::splat(gamedata.player_size)), // hmmmm
                texture: asset_server.load(sheet.path),
                atlas: TextureAtlas { 
                    index: sheet.first_animation_index,
                    layout: texture_atlases.add(TextureAtlasLayout::from_grid(
                        Vec2::new(sheet.width, sheet.height), 
                        sheet.columns, 
                        sheet.rows,
                        Some(Vec2::new(sheet.padding_x, sheet.padding_y)), 
                        None
                    ))
                },
                ..default()
            },
        ));
    }
}

