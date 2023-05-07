use bevy::prelude::*;

#[derive(Component)]
pub struct Gun {
    pub pattern: AimPattern,
    pub bullet_size: f32,
    pub bullet_vel: f32,
    pub bullet_damage: i32,
    pub color: Color,
    pub rotation: Quat,
    pub timer: Timer
}

#[derive(Component)]
pub enum AimPattern {
    Rotate, Snap, Spiral,
}
