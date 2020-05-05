mod actors;

use std::vec::Vec;
use ggez;
use ggez::event;
use ggez::event::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::{ conf, Context, graphics, GameResult, nalgebra as na };
use actors::projectile::ProjectileActor;
use actors::ship::ShipActor;


//TODO: Go over unnecessary &mut
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 400.0;



struct Main {
    ship: ShipActor,
    projectiles: Vec<ProjectileActor>,
}



impl Main {

    fn update_ship(&mut self, ctx: &mut Context) {

        self.ship.r#move(ctx);

        self.ship.shoot(&mut self.projectiles, ctx);

    }

    fn draw_ship(&mut self, ctx: &mut Context) -> GameResult {

        let ship_mesh = self.ship.draw_mesh(ctx)?;

        graphics::draw(ctx, &ship_mesh, (na::Point2::new(self.ship.pos_x, self.ship.pos_y),))?;

        Ok(())

    }

    fn update_projectiles(&mut self) {

        let mut outofbounds: Vec<usize> = Vec::new();

        for (i, projectile) in self.projectiles.iter_mut().enumerate() {

            projectile.r#move();

            if projectile.pos_y < -ProjectileActor::PROJECTILE_LENGTH {
                outofbounds.push(i);
            }

        }

        self.free_projectiles(outofbounds);

    }

    fn draw_projectiles(&mut self, ctx: &mut Context) -> GameResult {

        for projectile in &mut self.projectiles {

            let projectile_mesh = projectile.draw_mesh(ctx)?;

            graphics::draw(ctx, &projectile_mesh, (na::Point2::new(projectile.pos_x, projectile.pos_y),))?;

        }

        Ok(())

    }

    //TODO: Need to recode free
    fn free_projectiles(&mut self, outofbounds: Vec<usize>) {

        for i in outofbounds {
            self.projectiles.remove(i);
        }

        self.projectiles.shrink_to_fit();

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

        //TODO: multithread actors
        self.update_projectiles();
        self.update_ship(ctx);

        Ok(())

    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        //Set background colour of game
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 0.0));

        //Draw actors
        self.draw_ship(ctx)?;
        self.draw_projectiles(ctx)?;

        graphics::present(ctx)?;

        Ok(())

    }

    fn key_up_event(&mut self, _: &mut Context, key: KeyCode, _: KeyMods) {

        match key {

            KeyCode::J => {
                self.ship.speedburst_keyup();
            },

            _ => (),

        }

    }

    fn key_down_event(&mut self, _: &mut Context, _: KeyCode, _: KeyMods, _: bool) {
        //TODO
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
