pub mod projectile;
pub mod ship;
pub mod enemy;

use std::f32;
use ggez::{graphics, Context, GameResult};



//TODO: Figure out a different way to determine collisions (bounding box?)
//      Possibly filter out by distance first then determine if there's collision
pub fn is_collision<T, U>(first_actor: &T, second_actor: &U) -> bool
    where T: Actor, U: Actor {

        //Actor positions
        let (x0, y0) = first_actor.get_position();
        let (x1, y1) = second_actor.get_position();

        let imminent_collision: bool = ((x0 - x1).powi(2) + (y0 - y1).powi(2)).sqrt() < 30.0;

        if !imminent_collision { return imminent_collision }

        let has_collided: bool = (|| {
            ((x0 - x1).powi(2) + (y0 - y1).powi(2)).sqrt() < 5.0
        })();

        has_collided

}



pub trait Actor {

    fn r#move(&mut self, ctx: &mut Context);

    fn draw_mesh(&self, ctx: &mut Context) -> GameResult<graphics::Mesh>;

    fn get_position(&self) -> (f32, f32);

}
