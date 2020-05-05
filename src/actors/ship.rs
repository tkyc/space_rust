use std::f32;
use std::time::Instant;
use ggez::{ graphics, Context, GameResult, nalgebra as na };
use ggez::input::keyboard;
use ggez::event::KeyCode;
use super::projectile::ProjectileActor;



pub struct ShipActor {
    pub pos_x: f32,
    pub pos_y: f32,
    pub angle: f32,
    deltav: f32,
    lastshot: Instant,
    speedburst: (Instant, bool),
}



impl ShipActor {

    const VELOCITY: f32 = 2.0;

    const VELOCITY_LIMIT: f32 = 20.0;

    const ACCELERATION: f32 = 1.1;

    const SHOT_DELAY: u128 = 200; //ms

    const BURST_DELAY: u128 = 1000; //ms

    //Positioning offset from drawn ref points
    const DEFAULT_POS_X: f32 = crate::WINDOW_WIDTH / 2.0;
    const DEFAULT_POS_Y: f32 = crate::WINDOW_HEIGHT / 2.0 + 100.0;


    #[allow(dead_code)]
    pub fn new(pos_x: f32, pos_y: f32) -> ShipActor {
        ShipActor {
            pos_x: pos_x,
            pos_y: pos_y,
            angle: 0.0,
            deltav: ShipActor::VELOCITY,
            lastshot: Instant::now(),
            speedburst: (Instant::now(), false),
        }
    }

    pub fn r#move(&mut self, ctx: &mut Context) {

        //Multiple ifs for simultaneous key presses
        if keyboard::is_key_pressed(ctx, KeyCode::W) {

            let (y, x) = self.get_direction_vector();

            self.speedburst_keydown(ctx);

            self.pos_x += x * self.deltav;
            self.pos_y -= y * self.deltav;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::S) {

            //Backpedal is slower and can't speed burst
            let (y, x) = self.get_direction_vector();

            self.pos_x -= x;
            self.pos_y += y;

        }

        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.angle += 0.05;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.angle -= 0.05;
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

    //Get the current direction the ship is facing
    pub fn get_direction_vector(&self) -> (f32, f32) {
        (self.angle.cos(), self.angle.sin())
    }

    //Drawn ref points -- mesh is drawn with origin as ref point
    pub fn draw_mesh(&self, ctx: &mut Context) -> GameResult<graphics::Mesh> {

        let rot = na::geometry::Rotation2::new(self.angle);

        let orientation = [rot * na::Point2::new(0.0, 0.0),
                           rot * na::Point2::new(-10.0, 30.0),
                           rot * na::Point2::new(10.0, 30.0)];

        graphics::Mesh::from_triangles(
            ctx,
            &orientation,
            graphics::WHITE,
        )

    }

    //TODO: Speed burst bar
    //On KeyCode::J up deaccelerate and reset speed burst timer
    pub fn speedburst_keyup(&mut self) {

        self.reset_speedburst_timer();

        self.deaccelerate();

    }

    //On KeyCode::J down accelerate the ship to max velocity
    fn speedburst_keydown(&mut self, ctx: &mut Context) {

        let (_, can_burst) = self.speedburst;

        if keyboard::is_key_pressed(ctx, KeyCode::J) && can_burst {

            self.accelerate();

        } else {

            self.deaccelerate();

        }

    }

    //On completion of a full speed burst or partial speed burst -- reset the timer
    fn reset_speedburst_timer(&mut self) {
        self.speedburst = (Instant::now(), false);
    }

    fn accelerate(&mut self) {

        self.deltav *= ShipActor::ACCELERATION;

        if self.deltav >= ShipActor::VELOCITY_LIMIT {

            self.reset_speedburst_timer();

        }

    }

    fn deaccelerate(&mut self) {

        let (last_burst, can_burst) = &mut self.speedburst;

        self.deltav = ShipActor::VELOCITY;

        //Time elapsed since last speed burst must be greater than the delay
        if last_burst.elapsed().as_millis() >= ShipActor::BURST_DELAY {

            *can_burst = true;

        }

    }

}



impl Default for ShipActor {

    fn default() -> ShipActor {
        ShipActor {
            pos_x: ShipActor::DEFAULT_POS_X,
            pos_y: ShipActor::DEFAULT_POS_Y,
            angle: 0.0,
            deltav: ShipActor::VELOCITY,
            lastshot: Instant::now(),
            speedburst: (Instant::now(), false),
        }
    }

}
