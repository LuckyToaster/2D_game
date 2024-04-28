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

        if k.pressed(KeyCode::KeyW) { 
            direction += forward; 
            *state = AnimationState::Moving;
        }

        if k.pressed(KeyCode::KeyA) { 
            direction -= right; 
            *state = AnimationState::Moving;
        }

        if k.pressed(KeyCode::KeyS) { 
            direction -= forward; 
            *state = AnimationState::Moving;
        }

        if k.pressed(KeyCode::KeyD) { 
            direction += right; 
            *state = AnimationState::Moving;
        }

        if k.pressed(KeyCode::KeyL) { 
            rotation_factor += 1.0; 
            *state = AnimationState::TurningLeft;
        }

        if k.pressed(KeyCode::Quote) { 
            rotation_factor -= 1.0; 
            *state = AnimationState::TurningRight;
        }

        if direction.length() > 0.0 { 
            direction = direction.normalize(); 
        }

        let rotation = rotation_factor * gamedata.player_rotation_speed * time.delta_seconds();
        pt.rotate_z(rotation);
        pt.translation += direction * gamedata.player_speed * time.delta_seconds();
    }
}
