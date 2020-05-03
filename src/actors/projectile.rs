use ggez::{graphics, Context, GameResult};
use super::ship::ShipActor;



pub struct ProjectileActor {
    pub pos_x: f32,
    pub pos_y: f32,
    vertices: [[f32; 2]; 2],
}



impl ProjectileActor {

    //Velocity of projectile
    pub const VELOCITY: f32 = 3.0;

    pub const PROJECTILE_LENGTH: f32 = 20.0;

    //Drawn ref point -- mesh is drawn with origin as ref point
    const DEFAULT_ORIENTATION: [[f32; 2]; 2] = [[0.0, 0.0], [0.0, -ProjectileActor::PROJECTILE_LENGTH]];

    pub fn new(ship: &ShipActor) -> ProjectileActor {
        ProjectileActor {
            //Need ship position to orient projectile in front of ship
            pos_x: ship.pos_x,
            pos_y: ship.pos_y - 10.0,
            vertices: ProjectileActor::DEFAULT_ORIENTATION,
        }
    }

    pub fn draw_projectile(&mut self, ctx: &mut Context) -> GameResult<graphics::Mesh> {
        graphics::Mesh::new_line(
            ctx,
            &self.vertices,
            1.0,
            graphics::WHITE,
        )
    }

}
