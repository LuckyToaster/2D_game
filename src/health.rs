use crate::player::Player;
use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub i32);

pub fn handle(mut commands: Commands, health_q: Query<(Entity, &Health)>) {
    for (entity, health ) in &health_q {
        if health.0 <= 0 { 
            commands.entity(entity).despawn(); 
        }
    }
}

pub fn quit_on_player_death(
    mut exit: EventWriter<AppExit>, 
    player_health_query: Query<&Health, With<Player>>
) {
    if let Ok(health) = player_health_query.get_single() {
        if health.0 <= 0 {
            exit.send(AppExit);
        }
    }
}

