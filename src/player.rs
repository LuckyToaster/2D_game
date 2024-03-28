use crate::health::Health;
use crate::gamedata::GameData;
use crate::guns::{
    Guns, 
    Gun
};

use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    }, 
    prelude::*, 
};


// =======
// STRUCTS 
// =======

pub const SIZE: f32 = 20.0;
pub const SPEED: f32 = 175.0;
pub const ROTATION_SPEED: f32 = 3.5;

#[derive(Component)]
pub struct PlayerCamera;


#[derive(Component)]
pub struct Player {
    hitbox_size: f32,
    speed: f32,
    rotation_speed: f32,
}

impl Player {
    pub fn new(hitbox_size: f32, speed: f32, rotation_speed: f32) -> Self {
        Self { hitbox_size, speed, rotation_speed }
    }

    pub fn default() -> Self {
        Self { hitbox_size: 17.0, speed: 180.0, rotation_speed: 3.75 }
    }
}


// =======
// SYSTEMS
// =======

pub fn spawn_player_and_camera( 
    mut commands: Commands,
    gamedata: Res<GameData>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        PlayerCamera,
        BloomSettings::default(), // 3. Enable bloom for the camera,
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::AcesFitted, //Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
    ));

    let texture = asset_server.load("Tilemap/tilemap.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 12, 11, Some(Vec2::new(1.0, 1.0)), None);
    let texture_atlas_layout = texture_atlases.add(layout);
    let animation_indices = AnimationIndices { first: 84, last: 88};

    commands.spawn((
        Player::default(),
        Health(3000),
        Guns::new(vec![Gun::player_gun(), /*Gun::default_snap(EntityType::Enemy)*/]),
        crate::player::AnimationIndices { first: 84, last: 88 },
        crate::player::AnimationTimer(
            Timer::from_seconds(
                0.1, 
                TimerMode::Repeating
            )
        ),
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas { 
                layout: texture_atlas_layout, 
                index: animation_indices.first
            },
            transform: Transform::from_scale(Vec3::splat(gamedata.player_size)),
            ..default()
        },
    ));
}


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
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


pub fn handle_movement_and_camera(
    gamedata: Res<GameData>,
    time: Res<Time>,
    k: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<PlayerCamera>)>,
) {
    if let (Ok(mut pt), Ok(mut ct)) = (player.get_single_mut(), camera.get_single_mut()) {
        let mut direction = Vec3::ZERO;
        let mut rotation_factor = 0.0;
        let forward = pt.rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0));
        let right = pt.rotation.mul_vec3(Vec3::new(1.0, 0.0, 0.0));

        if k.pressed(KeyCode::KeyW) { 
            direction += forward; 
        }
        if k.pressed(KeyCode::KeyA) { 
            direction -= right; 
        }
        if k.pressed(KeyCode::KeyS) { 
            direction -= forward; 
        }
        if k.pressed(KeyCode::KeyD) { 
            direction += right; 
        }
        if k.pressed(KeyCode::KeyL) { 
            rotation_factor += 1.0; 
        }
        if k.pressed(KeyCode::Quote) { 
            rotation_factor -= 1.0; 
        }
        if direction.length() > 0.0 { 
            direction = direction.normalize();  // tf?
        }

        let rotation = rotation_factor * gamedata.player_rotation_speed * time.delta_seconds();
        pt.rotate_z(rotation);
        ct.rotate_z(rotation);
        pt.translation += direction * gamedata.player_speed * time.delta_seconds();
        ct.translation = pt.translation;
    }
}

