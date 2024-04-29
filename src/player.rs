use crate::health::Health;
use crate::gamedata::GameData;
use crate::animations::{SpriteSheetConfig, AnimationState, AnimationTimer};
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
        // sprites / animation stuff
        sheet.animations.clone(), 
        AnimationState::Prone, 
        AnimationTimer(Timer::from_seconds(sheet.duration_s, TimerMode::Repeating)),
        SpriteSheetBundle {
            transform: Transform::from_scale(Vec3::splat(gamedata.player_size)),
            texture: asset_server.load(sheet.path),
            atlas: TextureAtlas { 
                index: sheet.animations.0.get("Prone").unwrap().first,
                layout: texture_atlases.add(TextureAtlasLayout::from_grid(
                    Vec2::new(sheet.frame_width, sheet.frame_height), 
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


pub fn handle_movement(
    time: Res<Time>,
    gamedata: Res<GameData>,
    k: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    if let Ok((mut pt, mut state)) = player.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut rotation_factor = 0.0;
        let forward = pt.rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0));
        let right = pt.rotation.mul_vec3(Vec3::new(1.0, 0.0, 0.0));

        let a_press = k.pressed(KeyCode::KeyA);
        let w_press = k.pressed(KeyCode::KeyW);
        let s_press = k.pressed(KeyCode::KeyS);
        let d_press = k.pressed(KeyCode::KeyD);
        let l_press = k.pressed(KeyCode::KeyL);
        let quote_press = k.pressed(KeyCode::Quote);

        let a_rel = k.just_released(KeyCode::KeyA);
        let w_rel = k.just_released(KeyCode::KeyW);
        let s_rel = k.just_released(KeyCode::KeyS);
        let d_rel = k.just_released(KeyCode::KeyD);
        let l_rel = k.just_released(KeyCode::KeyL);
        let quote_rel = k.just_released(KeyCode::Quote);

        if w_press { direction += forward; } 
        if a_press { direction -= right; } 
        if s_press { direction -= forward; } 
        if d_press { direction += right; } 

        if l_press { 
            rotation_factor += 1.0; 
            *state = AnimationState::TurningLeft;
        } 

        if quote_press { 
            rotation_factor -= 1.0; 
            *state = AnimationState::TurningRight;
        } 

        if w_rel || a_rel || s_rel || d_rel { 
            *state = AnimationState::Prone; 
        }

        if a_press || w_press || s_press || d_press {
            *state = AnimationState::Moving;
        }
        
        if direction.length() > 0.0 { 
            direction = direction.normalize(); 
        }

        let rotation = rotation_factor * gamedata.player_rotation_speed * time.delta_seconds();
        pt.rotate_z(rotation);
        pt.translation += direction * gamedata.player_speed * time.delta_seconds();
    }
}
