use ggez::{graphics, Context, GameResult};
use super::ship::ShipActor;



pub struct ProjectileActor {
    pub pos_x: f32,
    pub pos_y: f32,
    deltav: f32,
    vertices: [[f32; 2]; 2],
}



impl ProjectileActor {

    pub const PROJECTILE_LENGTH: f32 = 20.0;

    //Velocity of projectile
    const VELOCITY: f32 = 3.0;

    //Change in velocity
    const ACCELERATION: f32 = 1.5;

    //Drawn ref point -- mesh is drawn with origin as ref point
    const DEFAULT_ORIENTATION: [[f32; 2]; 2] = [[0.0, 0.0], [0.0, -ProjectileActor::PROJECTILE_LENGTH]];

    pub fn new(ship: &ShipActor) -> ProjectileActor {
        ProjectileActor {
            //Need ship position to orient projectile in front of ship
            pos_x: ship.pos_x,
            pos_y: ship.pos_y - 10.0,
            deltav: 1.0,
            vertices: ProjectileActor::DEFAULT_ORIENTATION,
        }
    }

    pub fn r#move(&mut self) {

        self.pos_y -= ProjectileActor::VELOCITY + self.deltav;

        self.deltav *= ProjectileActor::ACCELERATION;

    }

    pub fn draw_mesh(&mut self, ctx: &mut Context) -> GameResult<graphics::Mesh> {
        graphics::Mesh::new_line(
            ctx,
            &self.vertices,
            1.0,
            graphics::WHITE,
        )
    }

}
