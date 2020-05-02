use ggez;
use ggez::event;
use ggez::event::KeyCode;
use ggez::{conf, Context, graphics, GameResult, nalgebra as na};
use ggez::input::keyboard;
use std::vec::Vec;



const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 400.0;



struct ProjectileActor {
    pos_x: f32,
    pos_y: f32,
    vertices: [[f32; 2]; 2],
}



impl ProjectileActor {

    const PROJECTILE_LENGTH: f32 = 20.0;

    //Drawn ref point -- mesh is drawn with origin as ref point
    const DEFAULT_ORIENTATION: [[f32; 2]; 2] = [[0.0, 0.0], [0.0, -ProjectileActor::PROJECTILE_LENGTH]];

    //Velocity of projectile
    const VELOCITY: f32 = 3.0;

    fn new(ship: &ShipActor) -> ProjectileActor {
        ProjectileActor {
            pos_x: ship.pos_x,
            pos_y: ship.pos_y - 10.0,
            vertices: ProjectileActor::DEFAULT_ORIENTATION,
        }
    }

    fn draw_projectile(&mut self, ctx: &mut Context) -> GameResult<graphics::Mesh> {
        graphics::Mesh::new_line(
            ctx,
            &self.vertices,
            1.0,
            graphics::WHITE,
        )
    }

}



struct ShipActor {
    pos_x: f32,
    pos_y: f32,
    vertices: [[f32; 2]; 3],
}



impl ShipActor {

    //Positioning offset from drawn ref points
    const DEFAULT_POS_X: f32 = WINDOW_WIDTH / 2.0;
    const DEFAULT_POS_Y: f32 = WINDOW_HEIGHT / 2.0 + 300.0;

    //Drawn ref points -- meshes are drawn with origin as ref point
    const DEFAULT_ORIENTATION: [[f32; 2]; 3] = [[0.0, 0.0], [-10.0, 30.0], [10.0, 30.0]];
    const LEFT_ORIENTATION: [[f32; 2]; 3] = [[0.0, 0.0], [-5.0, 30.0], [10.0, 30.0]];
    const RIGHT_ORIENTATION: [[f32; 2]; 3] = [[0.0, 0.0], [-10.0, 30.0], [5.0, 30.0]];
    const BRAKE_ORIENTATION: [[f32; 2]; 3] = [[0.0, 0.0], [-20.0, 30.0], [20.0, 30.0]];

    fn r#move(&mut self, ctx: &mut Context) {

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.pos_y -= 1.0;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.pos_x -= 1.0;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.pos_y += 1.0;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.pos_x += 1.0;
        }

    }

    fn shoot(&mut self, projectiles: &mut Vec<ProjectileActor>, ctx: &mut Context) {
        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            projectiles.push(ProjectileActor::new(self));
        }
    }

    fn draw_ship(&mut self, ctx: &mut Context) -> GameResult<graphics::Mesh> {

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
            vertices: ShipActor::DEFAULT_ORIENTATION,
        }
    }

}



struct MainState {
    ship: ShipActor,
    projectiles: Vec<ProjectileActor>,
}



impl MainState {

    fn remove(&mut self, outofbounds_projectiles: Vec<usize>) {

        for i in outofbounds_projectiles {
            self.projectiles.remove(i);
        }

        self.projectiles.shrink_to_fit();
    }

}



impl event::EventHandler for MainState {

    fn update(&mut self, ctx: &mut Context) -> GameResult {

        //Update ships' current position
        self.ship.r#move(ctx);

        //Update positions of all current visible projectiles on screen
        let mut outofbounds_projectiles: Vec<usize> = Vec::new();

        for (i, projectile) in self.projectiles.iter_mut().enumerate() {

            projectile.pos_y -= ProjectileActor::VELOCITY;

            if projectile.pos_y < -ProjectileActor::PROJECTILE_LENGTH {
                outofbounds_projectiles.push(i);
            }

        }

        self.remove(outofbounds_projectiles);

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

    let state = &mut MainState {
        ship: Default::default(),
        projectiles: Vec::new(),
    };

    let window_settings = conf::WindowMode::default()
        .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT);

    ggez::graphics::set_mode(ctx, window_settings)?;

    event::run(ctx, event_loop, state)

}
