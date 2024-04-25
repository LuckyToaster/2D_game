use bevy::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;


#[derive(Copy, Clone, Deserialize)]
pub enum EntityType {
    Enemy, Player 
}

#[derive(Resource, Deserialize)]
pub struct HitboxSize(f32);


#[derive(Resource)]
pub enum GameState {
    Pause, InGame, Menu, Splash
}


#[derive(Resource, Deserialize)]
pub struct GameData {
    pub dt: f32,
    pub player_speed: f32,
    pub player_size: f32,
    pub player_rotation_speed: f32,
    pub width: f32,
    pub height: f32,
    pub scaling: i32,
}

impl Default for GameData {
    fn default() -> GameData {
        let file = File::open("config/gamedata.json").unwrap();
        let reader = BufReader::new(file);
        let data: GameData = serde_json::from_reader(reader).unwrap();
        data
    }
}

