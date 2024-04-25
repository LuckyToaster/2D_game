use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use bevy_math::{Vec2, Vec3};
use bevy::{
    utils::default,
    prelude::{ Deref, DerefMut }, 
    asset::{ AssetServer, Assets }, 
    time::{ Time, Timer, TimerMode }, 
    transform::components::Transform, 
    ecs::{ component::Component, system::{ Commands, Query, Res, ResMut } }, 
    sprite::{ SpriteSheetBundle, TextureAtlas, TextureAtlasLayout }, 
};


#[derive(Deserialize)]
pub struct SpriteSheet {
    pub path: String,
    pub width: f32,
    pub height: f32,
    pub columns: usize,
    pub rows: usize,
    pub first_animation_index: usize,
    pub last_animation_index: usize,
    pub padding_x: f32,
    pub padding_y: f32,
    pub duration_s: f32
}

impl SpriteSheet {
    pub fn player() -> Self {
        let file = File::open("config/player_sprites.json").unwrap();
        let reader = BufReader::new(file);
        let sheet: SpriteSheet = serde_json::from_reader(reader).unwrap();
        sheet 
    }
}

/*
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}
*/


/* 
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}


pub fn animate(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if atlas.index == indices.last { 
                atlas.index = indices.first;
            } else { 
                atlas.index += 1; 
            }
        }
    }
}
*/
