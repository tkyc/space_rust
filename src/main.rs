mod actors;

use std::collections::LinkedList;
use ggez;
use ggez::event;
use ggez::event::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::{ conf, Context, graphics, GameResult, nalgebra as na };
use actors::Actor;
use actors::projectile::ProjectileActor;
use actors::ship::ShipActor;
use actors::enemy::EnemyActor;


//TODO: Go over unnecessary &mut
//TODO: Refactor actors to implement traits for polymorphic calls
//TODO: Apply declarative paradigm
//TODO: Overflow not fixed for dmg calc.
//TODO: Check unwrap() docs (better way to do this?)
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 400.0;



struct Main {
    ship: ShipActor,
    projectiles: LinkedList<ProjectileActor>,
    enemies: LinkedList<EnemyActor>,
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

    fn update_projectiles(&mut self, ctx: &mut Context) {

        //200.0 offset b/c noticed pixel dimensions never change from 800*600 despite explicit definition -- bug?
        let inbounds = |p: &ProjectileActor| p.pos_x <= WINDOW_WIDTH &&
                                             p.pos_x >= 0.0 &&
                                             p.pos_y <= WINDOW_HEIGHT + 200.0 &&
                                             p.pos_y >= 0.0;

        (0..self.projectiles.len()).for_each(|_x| {

            match self.projectiles.pop_front() {

                Some(mut projectile) => {

                    if inbounds(&projectile) {

                        projectile.r#move(ctx);

                        self.projectiles.push_back(projectile);

                    }

                },

                None => (),

            };

        });

    }

    fn draw_projectiles(&mut self, ctx: &mut Context) -> GameResult {

        for projectile in &self.projectiles {

            let projectile_mesh = projectile.draw_mesh(ctx)?;

            graphics::draw(ctx, &projectile_mesh, (na::Point2::new(projectile.pos_x, projectile.pos_y),))?;

        }

        Ok(())

    }

    fn update_enemies(&mut self, ctx: &mut Context) {

        (0..self.enemies.len()).for_each(|_x| {

            match self.enemies.pop_front() {

                Some(mut enemy) => {

                    if !enemy.is_eliminated() {

                        enemy.face_player(&self.ship);

                        enemy.r#move(ctx);

                        self.enemies.push_back(enemy);

                    }

                },

                None => (),

            };

        });

    }

    fn draw_enemies(&mut self, ctx: &mut Context) -> GameResult {

        for enemy in &self.enemies {

            let enemy_mesh = enemy.draw_mesh(ctx)?;

            graphics::draw(ctx, &enemy_mesh, (na::Point2::new(enemy.pos_x, enemy.pos_y),))?;

        }

        Ok(())
    }

    fn update_collisions(&mut self) {

        for projectile in &self.projectiles {

            for enemy in &mut self.enemies {

                if actors::is_collision(projectile, enemy) {

                    enemy.hit();
                    //println!("Within radius");
                }

            }

        }

    }

}



impl Default for Main {

    fn default() -> Main {
        Main {
            ship: ShipActor::default(),
            projectiles: LinkedList::new(),
            enemies: LinkedList::new(),
        }
    }

}



impl event::EventHandler for Main {

    fn update(&mut self, ctx: &mut Context) -> GameResult {

        //TODO: multithread or async/await actors?
        self.update_projectiles(ctx);
        self.update_ship(ctx);
        self.update_enemies(ctx);
        self.update_collisions();

        Ok(())

    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        //Set background colour of game
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 0.0));

        //Draw actors
        self.draw_ship(ctx)?;
        self.draw_projectiles(ctx)?;
        self.draw_enemies(ctx)?;

        graphics::present(ctx)?;

        Ok(())

    }

    fn key_up_event(&mut self, _: &mut Context, key: KeyCode, _: KeyMods) {

        match key {

            KeyCode::J => {
                self.ship.speedburst_keyup();
                self.enemies.push_back(EnemyActor::spawn()); //TODO: Testing -- remove
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
