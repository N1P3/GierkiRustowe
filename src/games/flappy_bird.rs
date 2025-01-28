use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use rand::Rng;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Pipe {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(PartialEq)]
pub struct FlappyBirdGame {
    pub position: (f32, f32),
    pub width: f32,
    pub height: f32,
    pub velocity: f32,
    pub pipes: Vec<Pipe>,
    pub pipe_speed: f32,
    pub pipe_gap: f32,
    pub is_dead: bool,
}

impl FlappyBirdGame {
    pub fn new() -> Self {
        FlappyBirdGame {
            position: (100.0, 300.0),
            width: 30.0,
            height: 30.0,
            velocity: 0.0,
            pipes: Vec::new(),
            pipe_speed: 4.0,
            pipe_gap: 250.0,
            is_dead: false,
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let bird_rect = graphics::Rect::new(self.position.0, self.position.1, self.width, self.height);
        let rect_mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(1.0), bird_rect, Color::RED)?;
        graphics::draw(ctx, &rect_mesh, DrawParam::default())?;

        for pipe in &self.pipes {
            let top_pipe_rect = graphics::Rect::new(pipe.x, 0.0, pipe.width, pipe.height);
            let bottom_pipe_rect = graphics::Rect::new(pipe.x, pipe.y, pipe.width, pipe.height);

            let top_mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(1.0), top_pipe_rect, Color::GREEN)?;
            let bottom_mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(1.0), bottom_pipe_rect, Color::GREEN)?;

            graphics::draw(ctx, &top_mesh, DrawParam::default())?;
            graphics::draw(ctx, &bottom_mesh, DrawParam::default())?;
        }

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.is_dead {
            return Ok(());
        }
        self.velocity += 0.1;
        self.position.1 += self.velocity;
        if self.position.1 < 0.0 || self.position.1 + self.height > 600.0 {
            self.is_dead = true;
        }
        for pipe in &mut self.pipes {
            pipe.x -= self.pipe_speed;
        }
        self.pipes.retain(|pipe| pipe.x + pipe.width > 0.0);
        if self.pipes.is_empty() || self.pipes[self.pipes.len() - 1].x < 600.0 {
            self.generate_pipe();
        }
        self.check_collisions(ctx);
        Ok(())
    }

    pub fn flap(&mut self) {
        if !self.is_dead {
            self.velocity = -3.0;
        }
    }

    fn generate_pipe(&mut self) {
        let mut rng = rand::thread_rng();
        let pipe_height = rng.gen_range(100.0..400.0);
        let pipe = Pipe {
            x: 800.0,
            y: pipe_height + self.pipe_gap,
            width: 50.0,
            height: pipe_height,
        };
        self.pipes.push(pipe);
    }

    fn check_collisions(&mut self, ctx: &mut Context) {
        for pipe in &self.pipes {
            let bird_rect = graphics::Rect::new(self.position.0, self.position.1, self.width, self.height);
            let top_pipe_rect = graphics::Rect::new(pipe.x, 0.0, pipe.width, pipe.height);
            let bottom_pipe_rect = graphics::Rect::new(pipe.x, pipe.y, pipe.width, pipe.height);

            if bird_rect.overlaps(&top_pipe_rect) || bird_rect.overlaps(&bottom_pipe_rect) {
                self.is_dead = true;
                break;
            }
        }
    }

}
