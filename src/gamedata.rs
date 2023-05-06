use bevy::prelude::*;

#[derive(Resource)]
pub struct GameData {
    pub dt: f32,
    pub width: f32,
    pub height: f32,
    //pub txt_style: TextStyle
}

impl Default for GameData {
    fn default() -> GameData {
        GameData {
            dt: 1.0 / 240.0,
            width: 1200.0,
            height: 600.0,
        }
    }
}

