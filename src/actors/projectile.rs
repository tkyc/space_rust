use ggez::{ graphics, Context, GameResult, nalgebra as na };
use super::ship::ShipActor;



pub struct ProjectileActor {
    pub pos_x: f32,
    pub pos_y: f32,
    pub angle: f32,
    deltav: f32,
}



impl ProjectileActor {

    const PROJECTILE_LENGTH: f32 = 30.0;

    const VELOCITY: f32 = 1.0;

    //Change in velocity (change in deltav)
    const ACCELERATION: f32 = 1.2;

    pub fn new(ship: &ShipActor) -> ProjectileActor {
        ProjectileActor {
            //Need ship position to orient projectile in front of ship
            pos_x: ship.pos_x,
            pos_y: ship.pos_y,
            angle: ship.angle,
            deltav: ProjectileActor::VELOCITY,
        }

    }

    pub fn r#move(&mut self) {

        let (y, x) = (self.angle.cos(), self.angle.sin());

        self.pos_x += x * self.deltav;
        self.pos_y -= y * self.deltav;
        self.deltav *= ProjectileActor::ACCELERATION;

    }

    //Drawn ref point -- mesh is drawn with origin as ref point
    pub fn draw_mesh(&self, ctx: &mut Context) -> GameResult<graphics::Mesh> {

        let rot = na::geometry::Rotation2::new(self.angle);

        let orientation = [rot * na::Point2::new(0.0, 0.0),
                           rot * na::Point2::new(0.0, -ProjectileActor::PROJECTILE_LENGTH)];

        graphics::Mesh::new_line(
            ctx,
            &orientation,
            1.0,
            graphics::WHITE,
        )
    }

}
