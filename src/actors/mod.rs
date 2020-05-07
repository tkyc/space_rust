pub mod projectile;
pub mod ship;
pub mod enemy;

use ggez::{graphics, Context, GameResult};



pub fn is_collision<T: Actor>(factor: &T, sactor: &T) -> bool {
    true
}



pub trait Actor {

    fn r#move(&mut self, ctx: &mut Context);

    fn draw_mesh(&self, ctx: &mut Context) -> GameResult<graphics::Mesh>;

}
