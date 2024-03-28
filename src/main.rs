#![allow(dead_code)]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod gamedata;
mod bullets;
mod health;
mod player;
mod enemies;
mod guns;
mod ui;

/*
    module layout:
        COMPONENTS, RESOURCES, DATA STRUCTURES, then
        SYSTEMS, then
        IMPLEMENTATIONS, then
        CONSTRUCTORS
 */


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
                player::spawn_player_and_camera, 
                enemies::spawn
            )
        )
        .add_systems(Update,
            (
                //health::quit_on_player_death,
                player::handle_movement_and_camera,
                player::animate, // change to animations::animate (for all entities with animation components or whatever)
                health::handle,
                (guns::enemy_guns, guns::player_guns, bullets::handle).after(health::handle),
                ui::update,
                bevy::window::close_on_esc


            )
        ).run();
}


