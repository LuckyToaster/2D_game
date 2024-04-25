use bevy::{
    ecs::{ component::Component, system::{ Query, Res } },
    prelude::{Deref, DerefMut},
    sprite::TextureAtlas,
    time::{Time, Timer},
};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;


#[derive(Deserialize)]
pub struct SpriteSheetConfig {
    pub path: String,
    pub width: f32,
    pub height: f32,
    pub columns: usize,
    pub rows: usize,
    pub first_animation_index: usize,
    pub last_animation_index: usize,
    pub padding_x: f32,
    pub padding_y: f32,
    pub duration_s: f32,
    // TODO: add config for what indices for moving in different directions... also must add animations for different generic actions that player and enemies/npcs do ...
    // will have to think about how to implement that without making the spaghetti worse
}

impl SpriteSheetConfig {
    pub fn player() -> Self {
        let reader = BufReader::new(File::open("config/player_sprites.json").unwrap());
        let sheet: SpriteSheetConfig = serde_json::from_reader(reader).unwrap();
        sheet
    }

    pub fn enemies() -> Vec<SpriteSheetConfig> {
        let reader = BufReader::new(File::open("config/enemy_sprites.json").unwrap());
        let sheets: Vec<SpriteSheetConfig> = serde_json::from_reader(reader).unwrap();
        sheets
    }
}


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if atlas.index == indices.last {
                atlas.index = indices.first;
            } else {
                atlas.index += 1;
            }
        }
    }
}
