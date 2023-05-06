use crate::boss::Boss;
use crate::player::Player;
use crate::health::Health;
use bevy::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::text::BreakLineOn;

pub fn spawn_ui(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    let white_style = TextStyle {
        font: asset_server.load("Minecraft.ttf"),
        font_size: 16.0,
        color: Color::WHITE,
    };

    let gold_style = TextStyle { 
        font: asset_server.load("Minecraft.ttf"),
        font_size: 16.0,
        color: Color::WHITE,
    };

    commands.spawn(
        TextBundle {
            text: Text {
                alignment: TextAlignment::Center,
                linebreak_behaviour: BreakLineOn::AnyCharacter,
                sections: vec![
                    TextSection::from_style(gold_style.clone()),
                    TextSection::new(" FPS ", white_style.clone()),
                    TextSection::new(" Player Health: ", white_style.clone()),
                    TextSection::from_style(gold_style.clone()),
                    TextSection::new(" Boss Health: ", white_style),
                    TextSection::from_style(gold_style)
                ]
            },
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::SpaceBetween,
                align_content: AlignContent::Stretch,
                align_items: AlignItems::Stretch,
                ..default()
            },
            ..default()
        }
    );
}


pub fn update(
    diagnostics: Res<Diagnostics>, 
    mut text_query: Query<&mut Text>, 
    player_health_query: Query<&Health, With<Player>>,
    boss_health_query: Query<&Health, With<Boss>>
) {
    for mut text in &mut text_query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[0].value = format!("{value:.2}");
            }
        }
        if let Ok(health) = player_health_query.get_single() {
            text.sections[3].value = health.0.to_string();
        }

        for health in &boss_health_query {
            text.sections[5].value = health.0.to_string();
        }
    }
}


    /*commands.spawn((
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font: asset_server.load("Minecraft.ttf"),
                font_size: 16.0,
                color: Color::GOLD,
            }),
            TextSection::new(
                " FPS ",
                TextStyle {
                    font: asset_server.load("Minecraft.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                " Player Health: ",
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
            TextSection::new(
                " Boss Health: ",
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
    ));*/

    /*commands.spawn(
        TextBundle {
            text: Text {
                alignment: TextAlignment::Center,
                linebreak_behaviour: BreakLineOn::AnyCharacter,
                sections: vec![
                    TextSection::from_style(gold_style.clone()),
                    TextSection::new(" FPS ", white_style.clone()),
                    TextSection::new(" Player Health: ", white_style.clone()),
                    TextSection::from_style(gold_style.clone()),
                    TextSection::new(" Boss Health: ", white_style),
                    TextSection::from_style(gold_style)
                ]
            },
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::SpaceBetween,
                align_content: AlignContent::Stretch,
                align_items: AlignItems::Stretch,
                ..default()
            },
            ..default()
        }
    );*/
