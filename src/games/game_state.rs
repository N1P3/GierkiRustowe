use ggez::{graphics, Context, GameResult};
use ggez::event::{KeyCode, KeyMods, EventHandler};
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
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            active_game: ActiveGame::Menu,
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
        let menu_text = graphics::Text::new("Press 1 for Flappy Bird\nPress 2 for Snake\nPress ESC to leave to main menu");
        graphics::draw(ctx, &menu_text, (ggez::mint::Point2 { x: 300.0, y: 300.0 },))?;
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
        graphics::clear(ctx, graphics::Color::BLUE);
        match &mut self.active_game {
            ActiveGame::Menu => self.draw_menu(ctx)?,
            ActiveGame::FlappyBird(game) => game.draw(ctx)?,
            ActiveGame::Snake(game) => game.draw(ctx)?,
        }
        graphics::present(ctx)?;
        Ok(())
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
