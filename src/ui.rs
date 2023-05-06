use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
//use crate::gamedata::GameData;
use crate::player::Player;
use crate::health::Health;
use bevy::prelude::*;

#[derive(Component)]
pub struct Fps;


pub fn spawn_fps_text(
    mut commands: Commands, 
    //gamedata: Res<GameData>,
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
            TextSection::new(
                "\tHealth: ",
                TextStyle {
                    font: asset_server.load("Minecraft.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("Minecraft.ttf"),
                font_size: 16.0,
                color: Color::GOLD,
            }),
        ]),
        Fps,
    ));
}


pub fn show_player_health(
    mut text_query: Query<&mut Text>, 
    player_health_query: Query<&Health, With<Player>>
) {
    for mut text in &mut text_query {
        if let Ok(health) = player_health_query.get_single() {
            text.sections[3].value = health.0.to_string();
        }
    }
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
