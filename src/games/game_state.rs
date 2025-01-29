use crate::games::flappy_bird::FlappyBirdGame;
use crate::games::snake::SnakeGame;
use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{Color, DrawMode, DrawParam, Font, Mesh, PxScale, Rect, Text, TextFragment};
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(PartialEq)]
pub enum ActiveGame {
    Menu,
    FlappyBird(FlappyBirdGame),
    Snake(SnakeGame),
    Leaderboard,
}

pub struct GameState {
    pub active_game: ActiveGame,
    flappy_button: Rect,
    snake_button: Rect,
    leaderboard_button: Rect,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            active_game: ActiveGame::Menu,
            flappy_button: Rect::new(250.0, 200.0, 300.0, 60.0),
            snake_button: Rect::new(250.0, 300.0, 300.0, 60.0),
            leaderboard_button: Rect::new(250.0, 400.0, 300.0, 60.0),
        }
    }

    fn start_flappy_bird(&mut self) {
        self.active_game = ActiveGame::FlappyBird(FlappyBirdGame::new());
    }

    fn start_snake(&mut self) {
        self.active_game = ActiveGame::Snake(SnakeGame::new(40));
    }

    fn return_to_menu(&mut self) {
        if let ActiveGame::FlappyBird(game) = &self.active_game {
            self.save_score("Flappy Bird", game.score);
        } else if let ActiveGame::Snake(game) = &self.active_game {
            self.save_score("Snake", game.score);
        }
        self.active_game = ActiveGame::Menu;
    }

    fn save_score(&self, game_name: &str, score: u32) {
        let mut scores = self.load_scores();

        let entry = scores.entry(game_name.to_string()).or_insert(Vec::new());
        entry.push(score);
        entry.sort_by(|a, b| b.cmp(a));
        entry.truncate(10);

        self.write_scores(&scores);
    }

    fn load_scores(&self) -> HashMap<String, Vec<u32>> {
        let path = Path::new("game_scores.txt");
        let mut scores = HashMap::new();

        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                for line in content.lines() {
                    let parts: Vec<&str> = line.split(" | Score: ").collect();
                    if parts.len() == 2 {
                        if let Ok(score) = parts[1].parse::<u32>() {
                            scores.entry(parts[0].to_string()).or_insert(Vec::new()).push(score);
                        }
                    }
                }
            }
        }

        for entry in scores.values_mut() {
            entry.sort_by(|a, b| b.cmp(a));
            entry.truncate(10);
        }

        scores
    }

    fn write_scores(&self, scores: &HashMap<String, Vec<u32>>) {
        let path = Path::new("game_scores.txt");
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();

        for (game, scores_list) in scores {
            for score in scores_list {
                writeln!(file, "{} | Score: {}", game, score).unwrap();
            }
        }
    }

    fn draw_leaderboard(&mut self, ctx: &mut Context) -> GameResult {
        self.active_game = ActiveGame::Leaderboard;
        graphics::clear(ctx, Color::from_rgb(255, 253, 208));

        let font = Font::default();

        let title = Text::new(
            TextFragment::new("LEADERBOARD")
                .font(font.clone())
                .scale(PxScale::from(50.0))
                .color(Color::BLACK),
        );
        let title_params = DrawParam::default().dest(Point2 { x: 250.0, y: 50.0 });
        graphics::draw(ctx, &title, title_params)?;

        let scores = self.load_scores();

        let mut y_position = 140.0;
        let row_height = 40.0;
        let x_offset_flappy = 200.0;
        let x_offset_snake = 500.0;

        if let Some(flappy_scores) = scores.get("Flappy Bird") {
            let game_name_text = Text::new(
                TextFragment::new("Flappy Bird")
                    .font(font.clone())
                    .scale(PxScale::from(30.0))
                    .color(Color::BLACK),
            );
            let game_name_text_params = DrawParam::default().dest(Point2 { x: x_offset_flappy - 44.0, y: y_position - 30.0 });
            graphics::draw(ctx, &game_name_text, game_name_text_params)?;

            for (_index, score) in flappy_scores.iter().enumerate() {
                let score_text = Text::new(
                    TextFragment::new(score.to_string())
                        .font(font.clone())
                        .scale(PxScale::from(30.0))
                        .color(Color::BLACK),
                );

                let score_text_params = DrawParam::default().dest(Point2 { x: x_offset_flappy, y: y_position });
                graphics::draw(ctx, &score_text, score_text_params)?;

                y_position += row_height;

                if y_position > 600.0 {
                    break;
                }
            }
        }

        y_position = 140.0;

        if let Some(snake_scores) = scores.get("Snake") {
            let game_name_text = Text::new(
                TextFragment::new("Snake")
                    .font(font.clone())
                    .scale(PxScale::from(30.0))
                    .color(Color::BLACK),
            );
            let game_name_text_params = DrawParam::default().dest(Point2 { x: x_offset_snake - 5.0, y: y_position - 30.0 });
            graphics::draw(ctx, &game_name_text, game_name_text_params)?;

            for (_index, score) in snake_scores.iter().enumerate() {
                let score_text = Text::new(
                    TextFragment::new(score.to_string())
                        .font(font.clone())
                        .scale(PxScale::from(30.0))
                        .color(Color::BLACK),
                );

                let score_text_params = DrawParam::default().dest(Point2 { x: x_offset_snake, y: y_position });
                graphics::draw(ctx, &score_text, score_text_params)?;

                y_position += row_height;

                if y_position > 600.0 {
                    break;
                }
            }
        }

        Ok(())
    }


    fn draw_menu(&self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(255, 253, 208));

        let font = Font::default();
        let title = Text::new(
            TextFragment::new("GAME MENU")
                .font(font.clone())
                .scale(PxScale::from(50.0))
                .color(Color::BLACK)
        );
        let info = Text::new(
            TextFragment::new("(Click game name or press 1 or 2 to play)")
                .font(font.clone())
                .scale(PxScale::from(15.0))
                .color(Color::BLACK)
        );

        let leaderboard_text = Text::new(
            TextFragment::new("Leaderboard")
                .font(font.clone())
                .scale(PxScale::from(30.0))
                .color(Color::BLACK),
        );


        let flappy_rect = Mesh::new_rectangle(ctx, DrawMode::fill(), self.flappy_button, Color::from_rgb(173, 216, 230))?;
        let snake_rect = Mesh::new_rectangle(ctx, DrawMode::fill(), self.snake_button, Color::from_rgb(100, 149, 237))?;
        let leaderboard_rect = Mesh::new_rectangle(ctx, DrawMode::fill(), self.leaderboard_button, Color::from_rgb(192, 192, 192))?;

        let flappy_text = Text::new(
            TextFragment::new("Flappy Bird")
                .font(font.clone())
                .scale(PxScale::from(30.0))
                .color(Color::BLACK)
        );
        let snake_text = Text::new(
            TextFragment::new("Snake")
                .font(font.clone())
                .scale(PxScale::from(30.0))
                .color(Color::BLACK)
        );
        let exit_text = Text::new(
            TextFragment::new("ESC - Exit to Main Menu")
                .font(font)
                .scale(PxScale::from(20.0))
                .color(Color::RED)
        );

        graphics::draw(ctx, &title, (Point2 { x: 280.0, y: 100.0 },))?;
        graphics::draw(ctx, &info, (Point2 { x: 240.0, y: 145.0 },))?;
        graphics::draw(ctx, &flappy_rect, (Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(ctx, &snake_rect, (Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(ctx, &flappy_text, (Point2 { x: 330.0, y: 220.0 },))?;
        graphics::draw(ctx, &snake_text, (Point2 { x: 360.0, y: 320.0 },))?;
        graphics::draw(ctx, &exit_text, (Point2 { x: 280.0, y: 400.0 },))?;
        graphics::draw(ctx, &leaderboard_rect, (Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(ctx, &leaderboard_text, (Point2 { x: 330.0, y: 420.0 },))?;

        Ok(())
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match &mut self.active_game {
            ActiveGame::Menu => Ok(()),
            ActiveGame::Leaderboard => Ok(self.draw_leaderboard(ctx)?),
            ActiveGame::FlappyBird(game) => game.update(),
            ActiveGame::Snake(game) => Ok(game.update(ctx)),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &mut self.active_game {
            ActiveGame::Menu => self.draw_menu(ctx)?,
            ActiveGame::FlappyBird(game) => {
                graphics::clear(ctx, Color::from_rgb(135, 206, 250));
                game.draw(ctx)?;
            }
            ActiveGame::Snake(game) => {
                graphics::clear(ctx, Color::from_rgb(135, 206, 250));
                game.draw(ctx)?;
            }
            ActiveGame::Leaderboard => self.draw_leaderboard(ctx)?,
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left && self.active_game == ActiveGame::Menu {
            if self.flappy_button.contains([x, y]) {
                self.start_flappy_bird();
            } else if self.snake_button.contains([x, y]) {
                self.start_snake();
            } else if self.leaderboard_button.contains([x, y]) {
                self.draw_leaderboard(_ctx).expect("TODO: panic message");
            }
        }
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        if self.active_game == ActiveGame::Menu {
            match keycode {
                KeyCode::Key1 => self.start_flappy_bird(),
                KeyCode::Key2 => self.start_snake(),
                _ => {}
            }
        } else {
            if keycode == KeyCode::Escape {
                self.return_to_menu();
            } else if keycode == KeyCode::Space {
                if let ActiveGame::FlappyBird(ref mut game) = self.active_game {
                    game.flap();
                }
            }

            if let ActiveGame::Snake(ref mut game) = self.active_game {
                game.change_direction(keycode);
            }
        }
    }
}
