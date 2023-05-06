use bevy::prelude::*;

#[derive(Resource)]
pub struct GameData {
    pub dt: f32,
    pub width: f32,
    pub height: f32,
    //pub txt_style: TextStyle
}

// so deriving works but 'impl Default for GameData' doesn't like arguments

impl Default for GameData {
    fn default(/*asset_server: Res<AssetServer>*/) -> GameData {
        GameData {
            dt: 1.0 / 240.0,
            width: 1200.0,
            height: 600.0,
            /*txt_style: TextStyle {
                font: asset_server.load("Minecraft.ttf"),
                font_size: 16.0,
                color: Color::WHITE,
            },*/
        }
    }
}

