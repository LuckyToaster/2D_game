use bevy::prelude::*;

#[derive(Component, PartialEq, PartialOrd)]
pub struct Health(pub i32);

pub fn handle(
    mut commands: Commands,
    health_query: Query<(Entity, &Health)>,
) {
    for (entity, health) in &health_query {
        if health.0 <= 0 { 
            commands.entity(entity).despawn(); 
        }
    }
}



