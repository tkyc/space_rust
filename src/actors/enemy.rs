use std::f32;
use rand::Rng;
use ggez::{ graphics, Context, GameResult, nalgebra as na };
use super::ship::ShipActor;



pub struct EnemyActor {
    pub pos_x: f32,
    pub pos_y: f32,
    pub angle: f32,
    deltav: f32,
    hitpoints: i8, //Fast hits may cause hp to go negative -- overflow so can't use unsigned primitives
}



impl EnemyActor {

    const VELOCITY: f32 = 0.2;

    const HITPOINTS: i8 = 5;

    #[allow(dead_code)]
    pub fn new(pos_x: f32, pos_y: f32, angle: f32) -> EnemyActor {
        EnemyActor {
            pos_x: pos_x,
            pos_y: pos_y,
            angle: angle,
            deltav: EnemyActor::VELOCITY,
            hitpoints: EnemyActor::HITPOINTS,
        }
    }

    pub fn spawn() -> EnemyActor {

        let mut rng = rand::thread_rng();

        EnemyActor {
            pos_x: rng.gen_range(0.0, crate::WINDOW_WIDTH),
            pos_y: rng.gen_range(0.0, crate::WINDOW_HEIGHT + 200.0),
            angle: 0.0,
            deltav: EnemyActor::VELOCITY,
            hitpoints: EnemyActor::HITPOINTS,
        }

    }

    //Call before moving enemy actor
    pub fn face_player(&mut self, ship: &ShipActor) {
        //Get the angle to face the player ship
        self.angle = -(self.pos_x - ship.pos_x).atan2(self.pos_y - ship.pos_y);
    }

    pub fn hit(&mut self) {
        self.hitpoints -= 1;
    }

    pub fn is_eliminated(&self) -> bool {
        self.hitpoints <= 0
    }

}



impl super::Actor for EnemyActor {

    fn r#move(&mut self, _ctx: &mut Context) {

        //Get direction vector to player ship
        let (y, x) = (self.angle.cos(), self.angle.sin());

        //Move towards player ship
        self.pos_x += x * self.deltav;
        self.pos_y -= y * self.deltav;

    }

    //Drawn ref point -- mesh is drawn with origin as ref point
    fn draw_mesh(&self, ctx: &mut Context) -> GameResult<graphics::Mesh> {

        let rot = na::geometry::Rotation2::new(self.angle);

        let orientation = [rot * na::Point2::new(0.0, 0.0),
                           rot * na::Point2::new(-10.0, 30.0),
                           rot * na::Point2::new(10.0, 30.0)];

        graphics::Mesh::from_triangles(
            ctx,
            &orientation,
            graphics::Color::from_rgb(255, 0, 0),
        )

    }

    fn get_position(&self) -> (f32, f32) {
        (self.pos_x, self.pos_y)
    }

}
