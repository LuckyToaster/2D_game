use bevy::prelude::*;


#[derive(Copy, Clone)]
pub enum EntityType {
    Enemy, Player 
}


#[derive(Resource)]
pub enum GameState {
    Pause, InGame,
}


#[derive(Resource)]
pub struct GameData {
    pub dt: f32,
    pub player_speed: f32,
    pub player_size: f32,
    pub player_rotation_speed: f32,
    pub width: f32,
    pub height: f32,
    pub scaling: i32,
}


// get single instance entities (like the player)'s consts here
// move to player compoenent
impl Default for GameData {
    fn default() -> GameData {
        GameData {
            dt: 1.0 / 240.0,
            player_speed: 150.0,
            player_size: (8 * 3) as f32,
            player_rotation_speed: 3.5,
            width: 1200.0,
            height: 600.0,
            scaling: 3,
        }
    }
}

