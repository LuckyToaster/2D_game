use crate::health::Health;
use crate::gun::Target;
use bevy::{
    prelude::*,
    utils::Duration,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    render::{color::Color, mesh::{Mesh, shape::Circle}},
    core_pipeline::{
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    },
};

pub const SIZE: f32 = 20.0;
pub const SPEED: f32 = 175.0;
pub const ROTATION_SPEED: f32 = 3.5;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct Player {
    //pub speed: f32,
    //pub size: f32,
    //pub rotation_speed: f32,
    pub bullet_size: f32,
    pub bullet_vel: f32,
    pub shooting_timer: Timer,
}


pub fn spawn_player_and_camera( 
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn((
        crate::player::PlayerCamera,
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

    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("Tilemap/tilemap.png"),
        Vec2::new(16.0, 16.0), 
        12, 
        11, 
        Some(Vec2::new(1.0, 1.0)), 
        None
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Health(30),
        Target::Player,
        crate::player::AnimationIndices { first: 84, last: 88 },
        crate::player::AnimationTimer(
            Timer::from_seconds(
                0.1, 
                TimerMode::Repeating
            )
        ),
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(84), // pass in first animation index
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        },
        Player {
            //speed: 150.0,
            //size: (8 * gamedata.scaling) as f32,
            //rotation_speed: 3.5,
            bullet_size: 2.0,
            bullet_vel: 400.0,
            shooting_timer: Timer::new(
                Duration::from_millis(250), 
                TimerMode::Repeating
            )
        }
    ));
}


pub fn animate(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlasSprite)>
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last { 
                indices.first 
            } else { 
                sprite.index + 1 
            };
        }
    }
}


pub fn handle_movement_and_camera(
    time: Res<Time>,
    k: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<PlayerCamera>)>,
) {
    if let (Ok(mut pt), Ok(mut ct)) = (player.get_single_mut(), camera.get_single_mut()) {
        let mut direction = Vec3::ZERO;
        let mut rotation_factor = 0.0;
        let forward = pt.rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0));
        let right = pt.rotation.mul_vec3(Vec3::new(1.0, 0.0, 0.0));

        if k.pressed(KeyCode::W) { direction += forward; }
        if k.pressed(KeyCode::A) { direction -= right; }
        if k.pressed(KeyCode::S) { direction -= forward; }
        if k.pressed(KeyCode::D) { direction += right; }
        if k.pressed(KeyCode::L) { rotation_factor += 1.0; }
        if k.pressed(KeyCode::Apostrophe) { rotation_factor -= 1.0; }
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
    k: Res<Input<KeyCode>>,
) {
    for (pt, p) in &player {
        if k.just_pressed(KeyCode::P) {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(p.bullet_size).into()).into(),
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

/*pub fn update_bloom_settings(
    mut camera: Query<(Entity, Option<&mut BloomSettings>), With<Camera>>,
    mut text: Query<&mut Text>,
    mut commands: Commands,
    keycode: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let bloom_settings = camera.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    match bloom_settings {
        (entity, Some(mut bloom_settings)) => {
            *text = "BloomSettings (Toggle: Space)\n".to_string();
            text.push_str(&format!("(Q/A) Intensity: {}\n", bloom_settings.intensity));
            text.push_str(&format!(
                "(W/S) Low-frequency boost: {}\n",
                bloom_settings.low_frequency_boost
            ));
            text.push_str(&format!(
                "(E/D) Low-frequency boost curvature: {}\n",
                bloom_settings.low_frequency_boost_curvature
            ));
            text.push_str(&format!(
                "(R/F) High-pass frequency: {}\n",
                bloom_settings.high_pass_frequency
            ));
            text.push_str(&format!(
                "(T/G) Mode: {}\n",
                match bloom_settings.composite_mode {
                    BloomCompositeMode::EnergyConserving => "Energy-conserving",
                    BloomCompositeMode::Additive => "Additive",
                }
            ));
            text.push_str(&format!(
                "(Y/H) Threshold: {}\n",
                bloom_settings.prefilter_settings.threshold
            ));
            text.push_str(&format!(
                "(U/J) Threshold softness: {}\n",
                bloom_settings.prefilter_settings.threshold_softness
            ));

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).remove::<BloomSettings>();
            }

            let dt = time.delta_seconds();

            if keycode.pressed(KeyCode::A) {
                bloom_settings.intensity -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::Q) {
                bloom_settings.intensity += dt / 10.0;
            }
            bloom_settings.intensity = bloom_settings.intensity.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::S) {
                bloom_settings.low_frequency_boost -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::W) {
                bloom_settings.low_frequency_boost += dt / 10.0;
            }
            bloom_settings.low_frequency_boost = bloom_settings.low_frequency_boost.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::D) {
                bloom_settings.low_frequency_boost_curvature -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::E) {
                bloom_settings.low_frequency_boost_curvature += dt / 10.0;
            }
            bloom_settings.low_frequency_boost_curvature =
                bloom_settings.low_frequency_boost_curvature.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::F) {
                bloom_settings.high_pass_frequency -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::R) {
                bloom_settings.high_pass_frequency += dt / 10.0;
            }
            bloom_settings.high_pass_frequency = bloom_settings.high_pass_frequency.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::G) {
                bloom_settings.composite_mode = BloomCompositeMode::Additive;
            }
            if keycode.pressed(KeyCode::T) {
                bloom_settings.composite_mode = BloomCompositeMode::EnergyConserving;
            }

            if keycode.pressed(KeyCode::H) {
                bloom_settings.prefilter_settings.threshold -= dt;
            }
            if keycode.pressed(KeyCode::Y) {
                bloom_settings.prefilter_settings.threshold += dt;
            }
            bloom_settings.prefilter_settings.threshold =
                bloom_settings.prefilter_settings.threshold.max(0.0);

            if keycode.pressed(KeyCode::J) {
                bloom_settings.prefilter_settings.threshold_softness -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::U) {
                bloom_settings.prefilter_settings.threshold_softness += dt / 10.0;
            }
            bloom_settings.prefilter_settings.threshold_softness = bloom_settings
                .prefilter_settings
                .threshold_softness
                .clamp(0.0, 1.0);
        }

        (entity, None) => {
            *text = "Bloom: Off (Toggle: Space)".to_string();

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).insert(BloomSettings::default());
            }
        }
    }
}*/

