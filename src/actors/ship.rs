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
    vertices: [[f32; 2]; 3],
}



impl ShipActor {

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

    pub fn new(pos_x: f32, pos_y: f32, delta: f32, lastshot: Instant, vertices: [[f32; 2]; 3]) -> ShipActor {
        ShipActor {
            pos_x,
            pos_y,
            delta,
            lastshot,
            vertices,
        }
    }

    pub fn r#move(&mut self, ctx: &mut Context) {

        let new_delta = self.delta;

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.pos_y -= new_delta;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.pos_x -= new_delta;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.pos_y += new_delta;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.pos_x += new_delta;
        }

        self.delta += ShipActor::ACCELERATION;

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

        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            return graphics::Mesh::from_triangles(
                ctx,
                &ShipActor::LEFT_ORIENTATION,
                graphics::WHITE,
            )
        }

        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            return graphics::Mesh::from_triangles(
                ctx,
                &ShipActor::RIGHT_ORIENTATION,
                graphics::WHITE,
            )
        }

        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            return graphics::Mesh::from_triangles(
                ctx,
                &ShipActor::BRAKE_ORIENTATION,
                graphics::WHITE,
            )
        }

        graphics::Mesh::from_triangles(
            ctx,
            &ShipActor::DEFAULT_ORIENTATION,
            graphics::WHITE,
        )

    }

}



impl Default for ShipActor {

    fn default() -> ShipActor {
        ShipActor {
            pos_x: ShipActor::DEFAULT_POS_X,
            pos_y: ShipActor::DEFAULT_POS_Y,
            delta: 0.0,
            lastshot: Instant::now(),
            vertices: ShipActor::DEFAULT_ORIENTATION,
        }
    }

}
