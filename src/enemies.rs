use crate::health::Health;
use crate::gamedata::*;
use crate::guns::{
    Gun, 
    Guns, 
};

use bevy::prelude::*;


#[derive(Component)]
pub struct Enemy;


pub fn spawn(
    //data: Res<GameData>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Tilemap/tilemap.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 12, 11, Some(Vec2::new(1.0, 1.0)), Some(Vec2::new(0.0, 0.0)));
    let texture_atlas_layout = texture_atlases.add(layout);
    let animation_indices = crate::player::AnimationIndices {first: 84, last: 88};

    commands.spawn((
        Enemy,
        Health(100),
        animation_indices,
        Guns::new(vec![
            Gun::default_rotate(EntityType::Player), 
            Gun::default_snap(EntityType::Player), 
            Gun::default_spiral(EntityType::Player)
        ]),
        crate::player::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)
        ),
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout, 
                index: 84 // cannot do animation indices.first because move, cannot do &animation_indices.first
            },
            transform: Transform {
                translation: Vec3::new(100.0, 100.0, 0.0), 
                rotation: Quat::default(), 
                scale: Vec3::splat(3.0),
            },
            ..default()
        }
    ));
}

