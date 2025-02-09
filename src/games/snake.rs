use ggez::event::KeyCode;
use ggez::graphics::{Color, DrawMode, Font, PxScale, Rect, Text, TextFragment};
use ggez::timer;
use ggez::{Context, GameResult};
use std::collections::VecDeque;

#[derive(PartialEq)]
pub struct SnakeGame {
    pub snake: VecDeque<(i32, i32)>,
    pub direction: (i32, i32),
    pub grid_size: i32,
    pub food: (i32, i32),
    pub time_since_last_update: f32,
    pub score: u32,
    pub game_over: bool,
}

impl SnakeGame {
    pub fn new(grid_size: i32) -> Self {
        let mut snake = VecDeque::new();
        snake.push_front((5, 5));
        SnakeGame {
            snake,
            direction: (1, 0),
            grid_size,
            food: (10, 10),
            time_since_last_update: 0.0,
            score: 0,
            game_over: false,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        if self.game_over {
            return;
        }

        let delta_time = timer::delta(ctx).as_secs_f32();
        self.time_since_last_update += delta_time;

        if self.time_since_last_update >= 0.25 {
            self.time_since_last_update = 0.0;

            let new_head = (self.snake.front().unwrap().0 + self.direction.0,
                            self.snake.front().unwrap().1 + self.direction.1);

            self.snake.push_front(new_head);
            self.snake.pop_back();

            if self.snake.front().unwrap() == &self.food {
                self.score += 10;
                self.snake.push_back(*self.snake.back().unwrap());
                self.generate_food();
            }

            if self.is_game_over() {
                self.game_over = true;
            }
        }
    }

    pub fn change_direction(&mut self, keycode: KeyCode) {
        if self.game_over {
            return;
        }

        match keycode {
            KeyCode::Up => if self.direction != (0, 1) { self.direction = (0, -1) },
            KeyCode::Down => if self.direction != (0, -1) { self.direction = (0, 1) },
            KeyCode::Left => if self.direction != (1, 0) { self.direction = (-1, 0) },
            KeyCode::Right => if self.direction != (-1, 0) { self.direction = (1, 0) },
            _ => {}
        }
    }

    fn generate_food(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        loop {
            let x = rng.gen_range(0..self.grid_size);
            let y = rng.gen_range(0..self.grid_size - 10);

            if !self.snake.contains(&(x, y)) {
                self.food = (x, y);
                break;
            }
        }
    }

    fn is_game_over(&self) -> bool {
        let head = self.snake.front().unwrap();
        if head.0 < 0 || head.0 >= self.grid_size || head.1 < 0 || head.1 >= self.grid_size - 10 {
            return true;
        }
        for segment in self.snake.iter().skip(2) {
            if segment == head {
                return true;
            }
        }
        false
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        use ggez::graphics;

        for &(x, y) in &self.snake {
            let rect = Rect::new(x as f32 * 20.0, y as f32 * 20.0, 20.0, 20.0);
            let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?;
            graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }

        let food_rect = Rect::new(self.food.0 as f32 * 20.0, self.food.1 as f32 * 20.0, 20.0, 20.0);
        let food_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), food_rect, Color::RED)?;
        graphics::draw(ctx, &food_mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        if self.game_over {
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
            let exit_text = Text::new(
                TextFragment::new(format!("Press ESC to leave"))
                    .font(Font::default().clone())
                    .scale(PxScale::from(30.0))
                    .color(Color::BLACK)
            );
            graphics::draw(ctx, &game_over, (ggez::mint::Point2 { x: 280.0, y: 100.0 },))?;
            graphics::draw(ctx, &score_text, (ggez::mint::Point2 { x: 280.0, y: 150.0 },))?;
            graphics::draw(ctx, &exit_text, (ggez::mint::Point2 { x: 280.0, y: 200.0 },))?;
        }

        Ok(())
    }
}
