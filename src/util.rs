#![allow(dead_code)]
use std::{f32::consts::PI, ops::{Sub, Mul, Add}};
use crate::bullet::Bullet;
use nalgebra::Vector2;
use rand::Rng;

#[inline]
pub fn r(high: f32) -> f32 {
   if high <= 0.0 { return high; }
   rand::thread_rng().gen_range(0..high as i32) as f32
}

#[inline]
pub fn randint(high: i32) -> i32 {
   return rand::thread_rng().gen_range(0..high);
}

#[inline]
pub fn rrng(min: f32, high: f32) -> f32 {
   if min == high { min }
   else { rand::thread_rng().gen_range(min as i32 .. high as i32) as f32 }
}

/*#[inline]
pub fn rcolor() -> Color {
    return Color::from_rgba(randint(255) as u8, randint(255) as u8, randint(255) as u8, 255);
}*/

/*#[inline]
pub fn distance(p1: &Particle, p2: &Particle) -> f32 {
   return ((p1.p.x - p2.p.x).powf(2.0) + (p1.p.y - p2.p.y).powf(2.0)).sqrt();
}

#[inline]
pub fn collide(p1: &Particle, p2: &Particle) -> bool {
   return distance(p1, p2) <= p1.r + p2.r;
}

#[inline]
pub fn v(x: f32, y: f32) -> Vector2<f32> {
   return Vector2::new(x, y);
}

#[inline]
pub fn overlap(p1: &Particle, p2: &Particle) -> f32 {
   let overlap = (p1.r + p2.r) - distance(p1, p2);
   if overlap > 0.0 {overlap} else {0.0}
}


pub fn solve_collision(p1: &mut Particle, p2: &mut Particle) {
   let contact_angle = p2.p.y - p1.p.y.atan2(p2.p.x - p1.p.x);
   let theta1 = p1.v.x.atan2(p1.v.y);
   let theta2 = p2.v.x.atan2(p2.v.y);
   let v1 = (p1.v.x.powf(2.0) + p1.v.y.powf(2.0)).sqrt();
   let v2 = (p2.v.x.powf(2.0) + p2.v.y.powf(2.0)).sqrt();
   let c_a_plus_half_pi = contact_angle + PI * 0.5;
   let t1_minus_c_a = theta1 - contact_angle;
   let second = t1_minus_c_a.sin() * v1;
   let first = ((t1_minus_c_a.cos() * v1 * (p1.m - p2.m) as f32) + 
      (2.0 * p2.m as f32 * v2 * (theta2 - contact_angle).cos())) / (p1.m + p2.m) as f32;

   p1.v.x = first * contact_angle.cos() + (c_a_plus_half_pi.cos() * second);
   p1.v.y = first * contact_angle.sin() + (c_a_plus_half_pi.sin() * second);
}

// TODO: only tested for x or y axis overlap and not both
pub fn solve_overlap(p1: &mut Particle, p2: &mut Particle) {
   let overlap = overlap(p1, p2);
   if overlap > 0.0 {
      let displacement_half = p2.p.sub(p1.p).normalize().mul(overlap * 0.5);
      p1.p = p1.p.sub(displacement_half);
      p2.p = p2.p.add(displacement_half);
   }
}
*/
