use crate::health::Health;
use crate::gamedata::GameData;
use crate::animations::{TopDownStates, AnimationState, AnimationTimer, SpriteSheetConfig};
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
        AnimationState::new(TopDownStates::Prone, false),
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
        let forward = pt.rotation.mul_vec3(Vec3::Y);
        let right = pt.rotation.mul_vec3(Vec3::X);

        let a_press = k.pressed(KeyCode::KeyA);
        let w_press = k.pressed(KeyCode::KeyW);
        let s_press = k.pressed(KeyCode::KeyS);
        let d_press = k.pressed(KeyCode::KeyD);
        let l_press = k.pressed(KeyCode::KeyL);
        let quote_press = k.pressed(KeyCode::Quote);
        let awsd_press = a_press || w_press || s_press || d_press;

        if w_press { direction += forward; } 
        if a_press { direction -= right; } 
        if s_press { direction -= forward; } 
        if d_press { direction += right; } 
        if quote_press { rotation_factor -= 1.0; }
        if l_press { rotation_factor += 1.0; }

        if quote_press { state.change_if_its_not(TopDownStates::TurningRight); } 
        else if l_press { state.change_if_its_not(TopDownStates::TurningLeft); } 
        else if awsd_press { state.change_if_its_not(TopDownStates::Moving); } 
        else { state.change_if_its_not(TopDownStates::Prone); } 

        if direction.length() > 0.0 { 
            direction = direction.normalize(); 
        }

        let rotation = rotation_factor * gamedata.player_rotation_speed * time.delta_seconds();
        pt.rotate_z(rotation);
        pt.translation += direction * gamedata.player_speed * time.delta_seconds();
    }
}
