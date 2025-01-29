use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Font, Mesh, PxScale, Text, TextFragment};
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
    pub score: u32,
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
            score: 0,
            is_dead: false,
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let bird_rect = graphics::Rect::new(self.position.0, self.position.1, self.width, self.height);
        let rect_mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(1.0), bird_rect, Color::RED)?;
        graphics::draw(ctx, &rect_mesh, DrawParam::default())?;

        for pipe in &self.pipes {
            let top_pipe_rect = graphics::Rect::new(pipe.x, 0.0, pipe.width, pipe.height);
            let top_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), top_pipe_rect, Color::GREEN)?;
            graphics::draw(ctx, &top_mesh, DrawParam::default())?;

            let bottom_pipe_rect = graphics::Rect::new(pipe.x, pipe.y, pipe.width, pipe.height);
            let bottom_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), bottom_pipe_rect, Color::GREEN)?;
            graphics::draw(ctx, &bottom_mesh, DrawParam::default())?;
        }

        if self.is_dead {
            let game_over = Text::new(
                TextFragment::new("GAME OVER")
                    .font(Font::default().clone())
                    .scale(PxScale::from(50.0))
                    .color(Color::BLACK)
            );
            let score_text = Text::new(
                TextFragment::new(format!("Score: {}", self.score))
                    .font(Font::default().clone())
                    .scale(PxScale::from(30.0))
                    .color(Color::BLACK)
            );
            graphics::draw(ctx, &game_over, (ggez::mint::Point2 { x: 280.0, y: 100.0 },))?;
            graphics::draw(ctx, &score_text, (ggez::mint::Point2 { x: 280.0, y: 150.0 },))?;
        }

        Ok(())
    }

    pub fn update(&mut self) -> GameResult {
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

        for pipe in &self.pipes {
            if pipe.x + pipe.width < self.position.0 {
                self.score += 1;
            }
        }

        self.check_collisions();
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

    fn check_collisions(&mut self) {
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