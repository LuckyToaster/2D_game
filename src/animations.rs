use serde::Deserialize;
use bevy::{
    prelude::{Deref, DerefMut}, 
    sprite::TextureAtlas, 
    time::{Time, Timer},
    ecs::{ 
        component::Component, 
        system::{ Query, Res } 
    }, 
};
use std::{
    io::BufReader,
    collections::HashMap, 
    fs::File
};

// TODO: Use the TypeState design pattern to define behaviour for different types of spritesheets passed through the config
//
// maybe implement astortion animations? --> would be cool

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

    pub fn new(json_path: &str) -> Self {
        let reader = BufReader::new(File::open(json_path).unwrap());
        let sheet = serde_json::from_reader(reader).unwrap();
        sheet
    }

    pub fn new_vec(json_path: &str) -> Vec<SpriteSheetConfig> {
        let reader = BufReader::new(File::open(json_path).unwrap());
        let sheets: Vec<SpriteSheetConfig> = serde_json::from_reader(reader).unwrap();
        sheets
    }

    pub fn player() -> Self {
        SpriteSheetConfig::new("config/player_sprites.json")
    }

    pub fn enemies() -> Vec<SpriteSheetConfig> {
        SpriteSheetConfig::new_vec("config/enemy_sprites.json")
    }
}


// honestly get rid of this and put directly in AnimationComponent
#[derive(Component, Deserialize, Deref, DerefMut, Clone)]
pub struct Animations(pub HashMap<String, Indices>); 

#[derive(Component, PartialEq, Clone, Copy)]
pub enum TopDownStates {
    Prone, 
    Moving, 
    TurningLeft, 
    TurningRight, 
    Hurt,
}

/* 
#[derive(Component, PartialEq, Clone, Copy)]
pub enum IsometricStates {
    Prone, Moving, NE, NW, SE, SW, North, South, East, West
}
*/

#[derive(Component)]
pub struct AnimationState {
    pub current: TopDownStates, // honestly change this to be a &str, allows more flexibility in making different animations
    pub has_changed: bool,
}

impl AnimationState {
    pub fn new(current: TopDownStates, has_changed: bool) -> AnimationState {
        AnimationState { current, has_changed } 
    }
    
    #[inline]
    pub fn is(&self, animation: TopDownStates) -> bool {
        self.current == animation
    }

    #[inline]
    pub fn change_if_its_not(&mut self, new: TopDownStates) {
        if self.current != new {
            self.current = new;
            self.has_changed = true;
        }
    }
}

// wtf is deref and derefmut
#[derive(Component, Deref, DerefMut, Clone)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deserialize, Clone)]
pub struct Indices {
    pub first: usize,
    pub last: usize,
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &Animations, &mut AnimationState)>,
) {
    for (mut timer, mut atlas, states, mut state) in &mut query {

        let indices = match state.current {
            TopDownStates::Hurt => states.0.get("Hurt").unwrap(),
            TopDownStates::Moving => states.0.get("Moving").unwrap(),
            TopDownStates::Prone => states.0.get("Prone").unwrap(),
            TopDownStates::TurningLeft => states.0.get("TurningLeft").unwrap(),
            TopDownStates::TurningRight => states.0.get("TurningRight").unwrap(),
        };

        if state.has_changed { 
            let duration = timer.duration();
            timer.tick(duration); 
            atlas.index = indices.first;
        } else { 
            timer.tick(time.delta()); 
        }

        if timer.finished() {
            if atlas.index >= indices.last { 
                atlas.index = indices.first; 
            } else { 
                atlas.index += 1; 
            }
        } 

        state.has_changed = false;
    }
}
