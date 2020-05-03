mod actors;

use ggez;
use ggez::event;
use ggez::{conf, Context, graphics, GameResult, nalgebra as na};
use actors::projectile::ProjectileActor;
use actors::ship::ShipActor;
use std::vec::Vec;



const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 400.0;



struct Main {
    ship: ShipActor,
    projectiles: Vec<ProjectileActor>,
}



impl Main {

    fn free_projectiles(&mut self, outofbounds: Vec<usize>) {

        for i in outofbounds {
            self.projectiles.remove(i);
        }

        self.projectiles.shrink_to_fit();

    }

    fn move_projectiles(&mut self) -> Vec<usize> {

        let mut outofbounds: Vec<usize> = Vec::new();

        for (i, projectile) in self.projectiles.iter_mut().enumerate() {

            projectile.r#move();

            if projectile.pos_y < -ProjectileActor::PROJECTILE_LENGTH {
                outofbounds.push(i);
            }

        }

        outofbounds

    }

    fn move_ship(&mut self, ctx: &mut Context) {
        self.ship.r#move(ctx);
    }

}



impl Default for Main {

    fn default() -> Main {
        Main {
            ship: ShipActor::default(),
            projectiles: Vec::new(),
        }
    }

}



impl event::EventHandler for Main {

    fn update(&mut self, ctx: &mut Context) -> GameResult {

        //Update positions of all current visible projectiles on screen (TODO: multithread this -- tons of projectiles cause lag)
        let outofbounds = self.move_projectiles();
        self.free_projectiles(outofbounds);

        //Update ships' current position
        self.move_ship(ctx);

        Ok(())

    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        //Set background colour of game
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 0.0));

        //ShipActor
        let ship_mesh = self.ship.draw_ship(ctx)?;
        self.ship.shoot(&mut self.projectiles, ctx);
        graphics::draw(ctx, &ship_mesh, (na::Point2::new(self.ship.pos_x, self.ship.pos_y),))?;

        //ProjectileActors
        for projectile in &mut self.projectiles {
            let projectile_mesh = projectile.draw_projectile(ctx)?;
            graphics::draw(ctx, &projectile_mesh, (na::Point2::new(projectile.pos_x, projectile.pos_y),))?;
        }

        graphics::present(ctx)?;

        Ok(())

    }

}



pub fn main() -> GameResult {

    let cb = ggez::ContextBuilder::new("Space Rust", "Tkyc");

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut Main::default();

    let window_settings = conf::WindowMode::default()
        .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT);

    ggez::graphics::set_mode(ctx, window_settings)?;

    event::run(ctx, event_loop, state)

}
