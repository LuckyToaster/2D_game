use bevy::{
    ecs::{ component::Component, system::{ Query, Res } },
    prelude::{Deref, DerefMut},
    sprite::TextureAtlas,
    time::{Time, Timer},
};
use serde::Deserialize;
use std::{collections::HashMap, fs::File};
use std::io::BufReader;


#[derive(Deserialize, Clone, Component)]
pub struct SpriteSheetConfig {
    pub path: String,
    pub frame_width: f32,
    pub frame_height: f32,
    pub columns: usize,
    pub rows: usize,
    pub padding_x: f32,
    pub padding_y: f32,
    pub duration_s: f32,
    pub animations: Animations
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


#[derive(Component, Deserialize, Clone)]
pub struct Animations(pub HashMap<String, AnimationIndices>); 

#[derive(Component, Deserialize, Clone)]
pub enum AnimationState {
    Prone, 
    Moving, 
    TurningLeft, 
    TurningRight, 
    Hurt
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deserialize, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}


pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &Animations, &AnimationState)>,
) {
    for (mut timer, mut atlas, states, state) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let indices: &AnimationIndices;
            match *state {
                AnimationState::Hurt => indices = states.0.get("Hurt").unwrap(),
                AnimationState::Moving => indices = states.0.get("Moving").unwrap(),
                AnimationState::Prone => indices = states.0.get("Prone").unwrap(),
                AnimationState::TurningLeft => indices = states.0.get("TurningLeft").unwrap(),
                AnimationState::TurningRight => indices = states.0.get("TurningRight").unwrap(),
            }

            if atlas.index >= indices.last {
                atlas.index = indices.first;
            } else {
                atlas.index += 1;
            }
        }
    }
}
