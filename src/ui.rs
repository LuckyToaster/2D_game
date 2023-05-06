use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
//use crate::gamedata::GameData;
use bevy::prelude::*;

#[derive(Component)]
pub struct Fps;


pub fn spawn_fps_text(
    mut commands: Commands, 
    //________gamedata: Res<GameData>,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font: asset_server.load("Minecraft.ttf"),
                font_size: 16.0,
                color: Color::GOLD,
            }),
            TextSection::new(
                " FPS: ",
                TextStyle {
                    font: asset_server.load("Minecraft.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
        ]),
        Fps,
    ));
}


pub fn fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<Fps>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[0].value = format!("{value:.2}");
            }
        }
    }
}
