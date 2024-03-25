use crate::health::Health;

use bevy_math::primitives::Circle;
use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    }, 
    prelude::*, render::{
        color::Color, 
        mesh::Mesh, 
    }, 
    sprite::{
        ColorMaterial, 
        MaterialMesh2dBundle
    }, 
    utils::Duration
};

pub const SIZE: f32 = 20.0;
pub const SPEED: f32 = 175.0;
pub const ROTATION_SPEED: f32 = 3.5;

#[derive(Component)]
pub struct PlayerCamera;


#[derive(Component)]
pub struct Player {
    pub bullet_size: f32,
    pub bullet_vel: f32,
    pub shooting_timer: Timer,
}


pub fn spawn_player_and_camera( 
    mut commands: Commands,
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
        Health(3000),
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
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        },
        Player {
            bullet_size: 2.0,
            bullet_vel: 400.0,
            shooting_timer: Timer::new(
                Duration::from_millis(250), 
                TimerMode::Repeating
            )
        }
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
            atlas.index = if atlas.index == indices.last { 
                indices.first 
            } else { 
                atlas.index + 1 
            };
        }
    }
}


pub fn handle_movement_and_camera(
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

        if k.pressed(KeyCode::KeyW) { direction += forward; }
        if k.pressed(KeyCode::KeyA) { direction -= right; }
        if k.pressed(KeyCode::KeyS) { direction -= forward; }
        if k.pressed(KeyCode::KeyD) { direction += right; }
        if k.pressed(KeyCode::KeyL) { rotation_factor += 1.0; }
        if k.pressed(KeyCode::Quote) { rotation_factor -= 1.0; }
        if direction.length() > 0.0 { direction = direction.normalize(); }

        let rotation = rotation_factor * ROTATION_SPEED * time.delta_seconds();
        pt.rotate_z(rotation);
        ct.rotate_z(rotation);
        pt.translation += direction * SPEED * time.delta_seconds();
        ct.translation = pt.translation;
    }
}


pub fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player: Query<(&Transform, &Player)>,
    k: Res<ButtonInput<KeyCode>>,
) {
    for (pt, p) in &player {
        if k.just_pressed(KeyCode::KeyP) {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(p.bullet_size)).into(),
                    material: materials.add(ColorMaterial::from(Color::rgb(6.25, 9.4, 9.1))),
                    transform: Transform {
                        translation: pt.translation,
                        rotation: pt.rotation,
                        ..default()
                    },
                    ..default()
                },
                crate::bullets::Bullet { 
                    vel: p.bullet_vel, 
                    size: p.bullet_size,
                    damage: 10,
                    source: crate::bullets::BulletSource::Player
                },
            ));      
        } 
    }
}
