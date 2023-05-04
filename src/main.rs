#![allow(dead_code)]
mod bullets;
mod gamedata;
mod player;
mod boss;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) 
        .init_resource::<gamedata::GameData>()
        .add_startup_system(boss::spawn)
        .add_startup_system(player::spawn_player_and_camera)
        .add_system(bullets::handle)
        .add_system(player::handle_movement_and_camera)
        .add_system(player::animate)
        .add_system(player::shoot)
        .add_system(boss::aim_at_player)
        .add_system(boss::attack_player)
        .run();
}


