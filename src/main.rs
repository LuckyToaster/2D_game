#![allow(dead_code)]

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod gamedata;
mod bullets;
mod health;
mod player;
mod boss;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) 
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .init_resource::<gamedata::GameData>()

        .add_startup_system(player::spawn_player_and_camera)
        .add_startup_system(ui::spawn_fps_text)
        .add_startup_system(boss::spawn)

        .add_system(ui::fps)
        .add_system(health::handle)
        .add_system(bullets::handle)
        .add_system(player::handle_movement_and_camera)
        .add_system(player::animate)
        .add_system(player::shoot)
        .add_system(boss::aim_at_player)
        .add_system(boss::shoot_player)
        .run();
}


