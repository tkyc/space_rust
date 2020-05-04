use std::time::Instant;
use ggez::{graphics, Context, GameResult};
use ggez::input::keyboard;
use ggez::event::KeyCode;
use super::projectile::ProjectileActor;



pub struct ShipActor {
    pub pos_x: f32,
    pub pos_y: f32,
    speedburst: (Instant, bool),
    lastshot: Instant,
    deltav: f32,
    vertices: [[f32; 2]; 3],
}



impl ShipActor {

    const VELOCITY: f32 = 1.0;

    const VELOCITY_LIMIT: f32 = 10.0;

    const ACCELERATION: f32 = 0.2;

    //Delay is in milliseconds
    const SHOT_DELAY: u128 = 200;

    //Delay is in milliseconds
    const BURST_DELAY: u128 = 1500;

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
            speedburst: (Instant::now(), false),
            lastshot: Instant::now(),
            deltav: 1.0,
            vertices: ShipActor::DEFAULT_ORIENTATION,
        }
    }

    pub fn r#move(&mut self, ctx: &mut Context) {

        //Multiple ifs for simultaneous key presses
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.speedburst_keydown(ctx);
            self.pos_y -= ShipActor::VELOCITY + self.deltav;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.speedburst_keydown(ctx);
            self.pos_x -= ShipActor::VELOCITY + self.deltav;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.speedburst_keydown(ctx);
            self.pos_y += ShipActor::VELOCITY + self.deltav;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.speedburst_keydown(ctx);
            self.pos_x += ShipActor::VELOCITY + self.deltav;
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

    pub fn draw_mesh(&mut self, ctx: &mut Context) -> GameResult<graphics::Mesh> {

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

        self.deltav += ShipActor::ACCELERATION;

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
            speedburst: (Instant::now(), false),
            lastshot: Instant::now(),
            deltav: 1.0,
            vertices: ShipActor::DEFAULT_ORIENTATION,
        }
    }

}
