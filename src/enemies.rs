
use crate::health::Health;
use crate::gamedata::*;
use crate::guns::{ GunConfigs, Guns };
use crate::animations::{AnimationIndices, AnimationTimer, SpriteSheetConfig};
use bevy::prelude::*;


#[derive(Component)]
pub struct Enemy;


pub fn spawn(
    gamedata: Res<GameData>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let gunconfigs_vec = GunConfigs::enemies();
    let sheets = SpriteSheetConfig::enemies();

    for (gunconfigs, sheet) in gunconfigs_vec.into_iter().zip(sheets.into_iter()) {
        let guns = Guns::from(gunconfigs);
        commands.spawn((
            Enemy,
            Health(100),
            guns,
            AnimationIndices { first: sheet.first_animation_index, last: sheet.last_animation_index }, // from here to the end is the animation data
            AnimationTimer(Timer::from_seconds(sheet.duration_s, TimerMode::Repeating)),
            SpriteSheetBundle {
                transform: Transform::from_scale(Vec3::splat(gamedata.player_size)), // hmmmm
                texture: asset_server.load(sheet.path),
                atlas: TextureAtlas { 
                    index: sheet.first_animation_index,
                    layout: texture_atlases.add(TextureAtlasLayout::from_grid(
                        Vec2::new(sheet.width, sheet.height), 
                        sheet.columns, 
                        sheet.rows,
                        Some(Vec2::new(sheet.padding_x, sheet.padding_y)), 
                        None
                    ))
                },
                ..default()
            },
        ));
    }
}

