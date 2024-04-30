#![allow(dead_code)]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod gamedata;
mod bullets;
mod health;
mod player;
mod enemies;
mod animations;
mod camera;
mod guns;
mod ui;


fn main() {
    App::new()
        .add_plugins( 
            (
                DefaultPlugins,
                FrameTimeDiagnosticsPlugin
                //FrameTimeDiagnosticsPlugin::default()
            )
        ) 
        .init_resource::<gamedata::GameData>()
        .add_systems(Startup, 
            (
                ui::spawn, 
                player::spawn, 
                enemies::spawn,
                camera::spawn
            )
        )
        .add_systems(Update,
            (
                health::quit_on_player_death,
                player::handle_movement,
                animations::animate.after(player::handle_movement),
                camera::follow_player,
                health::handle,
                (guns::enemy_guns, guns::player_guns, bullets::handle).after(health::handle),
                ui::update,
                bevy::window::close_on_esc
            )
        ).run();
}


