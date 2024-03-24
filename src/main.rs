#![allow(dead_code)]

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod gamedata;
mod bullets;
mod health;
mod player;
mod boss;
mod gun;
mod ui;


fn main() {
    App::new()
        .add_plugins( 
            (
                DefaultPlugins,
                //ImagePlugin::default_nearest(),
                FrameTimeDiagnosticsPlugin
                //FrameTimeDiagnosticsPlugin::default()
            )
        ) 
        .init_resource::<gamedata::GameData>()
        .add_systems(Startup, 
            (
                ui::spawn, 
                player::spawn_player_and_camera, 
                boss::spawn
            )
        )
        .add_systems(Update,
            (
                player::handle_movement_and_camera,
                player::animate, // change to animations::animate (for all entities with animation components or whatever)
                player::shoot,
                boss::aim_at_player,
                boss::shoot_player,
                bullets::handle,
                health::handle,
                health::quit_on_player_death,
                ui::update,
                bevy::window::close_on_esc
            )
        ).run();

        /* 
        .add_startup_system(player::spawn_player_and_camera)
        .add_startup_system(ui::spawn)
        .add_startup_system(boss::spawn)

        //.add_systems()

        .add_system(bevy::window::close_on_esc.before(bullets::handle))
        .add_system(health::quit_on_player_death.before(bullets::handle))
        .add_system(ui::update.before(bullets::handle))
        .add_system(health::handle.before(bullets::handle))
        .add_system(player::handle_movement_and_camera.before(bullets::handle))
        .add_system(player::animate.before(bullets::handle))
        .add_system(player::shoot.before(bullets::handle))
        .add_system(boss::aim_at_player.before(bullets::handle))
        .add_system(boss::shoot_player.before(bullets::handle))
        .add_system(bullets::handle)
        .run();
        */
}


