use std::time::Instant;
use ggez::{graphics, Context, GameResult};
use ggez::input::keyboard;
use ggez::event::KeyCode;
use super::projectile::ProjectileActor;



pub struct ShipActor {
    pub pos_x: f32,
    pub pos_y: f32,
    delta: f32,
    lastshot: Instant,
    accelerate: bool,
    vertices: [[f32; 2]; 3],
}



impl ShipActor {

    const VELOCITY: f32 = 1.0;

    const VELOCITY_LIMIT: f32 = 15.0;

    const ACCELERATION: f32 = 0.5;

    //Delay is in milliseconds
    const SHOT_DELAY: u128 = 300;

    //Positioning offset from drawn ref points
    const DEFAULT_POS_X: f32 = crate::WINDOW_WIDTH / 2.0;
    const DEFAULT_POS_Y: f32 = crate::WINDOW_HEIGHT / 2.0 + 300.0;

    //Drawn ref points -- meshes are drawn with origin as ref point
    const DEFAULT_ORIENTATION: [[f32; 2]; 3] = [[0.0, 0.0], [-10.0, 30.0], [10.0, 30.0]];
    const LEFT_ORIENTATION: [[f32; 2]; 3] = [[0.0, 0.0], [-5.0, 30.0], [10.0, 30.0]];
    const RIGHT_ORIENTATION: [[f32; 2]; 3] = [[0.0, 0.0], [-10.0, 30.0], [5.0, 30.0]];
    const BRAKE_ORIENTATION: [[f32; 2]; 3] = [[0.0, 0.0], [-20.0, 30.0], [20.0, 30.0]];

    pub fn new(pos_x: f32, pos_y: f32) -> ShipActor {
        ShipActor {
            pos_x: pos_x,
            pos_y: pos_y,
            delta: 1.0,
            lastshot: Instant::now(),
            accelerate: true,
            vertices: ShipActor::DEFAULT_ORIENTATION,
        }
    }

    pub fn r#move(&mut self, ctx: &mut Context) {

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.pos_y -= ShipActor::VELOCITY + self.delta;
            self.calculate_acceleration(ctx);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.pos_x -= ShipActor::VELOCITY + self.delta;
            self.calculate_acceleration(ctx);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.pos_y += ShipActor::VELOCITY + self.delta;
            self.calculate_acceleration(ctx);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.pos_x += ShipActor::VELOCITY + self.delta;
            self.calculate_acceleration(ctx);
        }

    }

    pub fn shoot(&mut self, projectiles: &mut Vec<ProjectileActor>, ctx: &mut Context) {

        //One sec. delay between shots
        if keyboard::is_key_pressed(ctx, KeyCode::Space)
            && self.lastshot.elapsed().as_millis() > ShipActor::SHOT_DELAY {

            projectiles.push(ProjectileActor::new(self));

            self.lastshot = Instant::now();

        }

    }

    pub fn draw_ship(&mut self, ctx: &mut Context) -> GameResult<graphics::Mesh> {

        let orientation: [[f32; 2]; 3] = if keyboard::is_key_pressed(ctx, KeyCode::A) { ShipActor::LEFT_ORIENTATION }
                                    else if keyboard::is_key_pressed(ctx, KeyCode::S) { ShipActor::BRAKE_ORIENTATION }
                                    else if keyboard::is_key_pressed(ctx, KeyCode::D) { ShipActor::RIGHT_ORIENTATION }
                                    else { ShipActor::DEFAULT_ORIENTATION };

        graphics::Mesh::from_triangles(
            ctx,
            &orientation,
            graphics::WHITE,
        )

    }

    fn calculate_acceleration(&mut self, ctx: &mut Context) {

        if keyboard::is_key_repeated(ctx) {

            //Accelerate
            if self.accelerate {
                self.delta += ShipActor::ACCELERATION;
                if self.delta >= ShipActor::VELOCITY_LIMIT {
                    self.accelerate = false;
                }
            }

            //Deaccelerate
            if !self.accelerate {
                self.delta += -ShipActor::ACCELERATION;
                if self.delta <= ShipActor::VELOCITY {
                    self.delta = ShipActor::VELOCITY;
                    self.accelerate = true;
                }
            }

        }

        if !keyboard::is_key_repeated(ctx) {
            self.delta = ShipActor::VELOCITY;
        }

    }

}



impl Default for ShipActor {

    fn default() -> ShipActor {
        ShipActor {
            pos_x: ShipActor::DEFAULT_POS_X,
            pos_y: ShipActor::DEFAULT_POS_Y,
            delta: 1.0,
            lastshot: Instant::now(),
            accelerate: true,
            vertices: ShipActor::DEFAULT_ORIENTATION,
        }
    }

}
