use ggez::{ graphics, Context, GameResult, nalgebra as na };
use rand::Rng;


pub struct EnemyActor {
    pub pos_x: f32,
    pub pos_y: f32,
    pub angle: f32,
    deltav: f32,
    hitpoints: u8,
}



impl EnemyActor {

    const VELOCITY: f32 = 1.0;

    const HITPOINTS: u8 = 5;

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

    pub fn r#move(&mut self, ctx: &mut Context) {
        //TODO
    }

    pub fn draw_mesh(&self, ctx: &mut Context) -> GameResult<graphics::Mesh> {

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

}