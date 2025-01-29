use ggez::{graphics, Context, GameResult};
use ggez::event::{KeyCode, KeyMods, EventHandler, MouseButton};
use ggez::graphics::{Color, Font, PxScale, Text, TextFragment, DrawMode, Rect, Mesh};
use ggez::mint::Point2;
use crate::games::flappy_bird::FlappyBirdGame;
use crate::games::snake::SnakeGame;

#[derive(PartialEq)]
pub enum ActiveGame {
    Menu,
    FlappyBird(FlappyBirdGame),
    Snake(SnakeGame),
}

pub struct GameState {
    pub active_game: ActiveGame,
    flappy_button: Rect,
    snake_button: Rect,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            active_game: ActiveGame::Menu,
            flappy_button: Rect::new(250.0, 200.0, 300.0, 60.0),
            snake_button: Rect::new(250.0, 300.0, 300.0, 60.0),
        }
    }

    fn start_flappy_bird(&mut self) {
        self.active_game = ActiveGame::FlappyBird(FlappyBirdGame::new());
    }

    fn start_snake(&mut self) {
        self.active_game = ActiveGame::Snake(SnakeGame::new(40));
    }

    fn return_to_menu(&mut self) {
        self.active_game = ActiveGame::Menu;
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
            TextFragment::new("(To enter a game, click on it's name or press a corresponding button)")
                .font(font.clone())
                .scale(PxScale::from(15.0))
                .color(Color::BLACK)
        );

        let flappy_rect = Mesh::new_rectangle(ctx, DrawMode::fill(), self.flappy_button, Color::from_rgb(173, 216, 230))?;
        let snake_rect = Mesh::new_rectangle(ctx, DrawMode::fill(), self.snake_button, Color::from_rgb(100, 149, 237))?;

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
        graphics::draw(ctx, &info, (Point2 { x: 115.0, y: 145.0 },))?;
        graphics::draw(ctx, &flappy_rect, (Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(ctx, &snake_rect, (Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(ctx, &flappy_text, (Point2 { x: 330.0, y: 220.0 },))?;
        graphics::draw(ctx, &snake_text, (Point2 { x: 360.0, y: 320.0 },))?;
        graphics::draw(ctx, &exit_text, (Point2 { x: 280.0, y: 400.0 },))?;

        Ok(())
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match &mut self.active_game {
            ActiveGame::Menu => Ok(()),
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
            }
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool
    ) {
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
