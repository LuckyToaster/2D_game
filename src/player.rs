use crate::health::Health;
use crate::gamedata::GameData;
use crate::animations::{SpriteSheetConfig, AnimationIndices, AnimationTimer};
use crate::guns::{
    Guns, 
    Gun
};
use bevy::prelude::*;

// =======
// STRUCTS 
// =======

//#[derive(Component)]
//pub struct PlayerCamera;


#[derive(Component)]
pub struct Player;


// =======
// SYSTEMS
// =======

pub fn spawn( 
    mut commands: Commands,
    gamedata: Res<GameData>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let sheet = SpriteSheetConfig::player();

    commands.spawn((
        Player,
        Health(3000),
        Guns::new(vec![Gun::player_gun()]),
        AnimationIndices { first: sheet.first_animation_index, last: sheet.last_animation_index }, // from here to the end is the animation data
        AnimationTimer(Timer::from_seconds(sheet.duration_s, TimerMode::Repeating)),
        SpriteSheetBundle {
            transform: Transform::from_scale(Vec3::splat(gamedata.player_size)),
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


/* 
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}
*/


pub fn handle_movement(
    time: Res<Time>,
    gamedata: Res<GameData>,
    k: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut pt) = player.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut rotation_factor = 0.0;
        let forward = pt.rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0));
        let right = pt.rotation.mul_vec3(Vec3::new(1.0, 0.0, 0.0));

        if k.pressed(KeyCode::KeyW) { direction += forward; }
        if k.pressed(KeyCode::KeyA) { direction -= right; }
        if k.pressed(KeyCode::KeyS) { direction -= forward; }
        if k.pressed(KeyCode::KeyD) { direction += right; }
        if k.pressed(KeyCode::KeyL) { rotation_factor += 1.0; }
        if k.pressed(KeyCode::Quote) { rotation_factor -= 1.0; }
        if direction.length() > 0.0 { direction = direction.normalize(); }

        let rotation = rotation_factor * gamedata.player_rotation_speed * time.delta_seconds();
        pt.rotate_z(rotation);
        pt.translation += direction * gamedata.player_speed * time.delta_seconds();
    }
}
