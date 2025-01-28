use ggez::{mint, Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::conf::{WindowMode, WindowSetup};
use crate::games::flappy_bird::FlappyBirdGame;


pub(crate) struct GameState {
    menu: bool,
    flappy_bird_game: Option<FlappyBirdGame>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            menu: true,
            flappy_bird_game: None,
        }
    }

    fn start_flappy_bird(&mut self) {
        self.menu = false;
        self.flappy_bird_game = Some(FlappyBirdGame::new());
    }

    fn return_to_menu(&mut self) {
        self.menu = true;
        self.flappy_bird_game = None;
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(ref mut game) = self.flappy_bird_game {
            game.update(ctx)?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));

        if self.menu {
            self.draw_menu(ctx)?;
        } else {
            if let Some(ref mut game) = self.flappy_bird_game {
                game.draw(ctx)?;
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        if self.menu {
            if keycode == KeyCode::Key1 {
                self.start_flappy_bird();
            }
        } else {
            if keycode == KeyCode::Escape {
                self.return_to_menu();
            } else if keycode == KeyCode::Space {
                if let Some(ref mut game) = self.flappy_bird_game {
                    game.flap();
                }
            }
        }
    }
}

impl GameState {
    fn draw_menu(&self, ctx: &mut Context) -> GameResult {
        let text = graphics::Text::new("Press 1 to Start Flappy Bird");

        let point = mint::Point2 { x: 200.0, y: 300.0 };

        graphics::draw(ctx, &text, (point,))?;

        Ok(())
    }
}
