pub mod projectile;
pub mod ship;
pub mod enemy;

use std::f32;
use ggez::{graphics, Context, GameResult};



//TODO: Figure out a different way to determine collisions (bounding box?)
pub fn is_collision<T, U>(first_actor: &T, second_actor: &U) -> bool
    where T: Actor, U: Actor {

        let (x0, y0) = first_actor.get_position();
        let (x1, y1) = second_actor.get_position();

        ((x0 - x1).powi(2) + (y0 - y1).powi(2)).sqrt() < 30.0

}



pub trait Actor {

    fn r#move(&mut self, ctx: &mut Context);

    fn draw_mesh(&self, ctx: &mut Context) -> GameResult<graphics::Mesh>;

    fn get_position(&self) -> (f32, f32);

}
