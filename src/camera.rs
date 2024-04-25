use bevy::{
    core_pipeline::{
        bloom::BloomSettings, 
        core_2d::Camera2dBundle, 
        tonemapping::Tonemapping
    }, 
    ecs::{
        query::{
            With, 
            Without
        }, 
        system::{
            Commands, 
            Query
        },
        component::Component, 
    }, 
    render::camera::Camera, 
    transform::components::Transform, 
    utils::default
};

use crate::player::Player;


#[derive(Component)]
pub struct GameCamera;


pub fn spawn(
    mut commands: Commands
) {
    commands.spawn((
        GameCamera,
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
}


pub fn follow_player(
    player_t: Query<&Transform, With<Player>>,
    mut camera_t: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    if let (Ok(pt), Ok(mut ct)) = (player_t.get_single(), camera_t.get_single_mut()) {
        ct.rotation = pt.rotation;
        ct.translation = pt.translation;         
    }
}

// TODO: pub fn transition_to
// TODO: snap to
    
   