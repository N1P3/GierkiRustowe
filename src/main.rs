use ggez::{ContextBuilder, event};
use ggez::conf::{WindowMode, WindowSetup};
use crate::games::game_state::GameState;

mod games;

fn main() -> ggez::GameResult {
    let (ctx, event_loop) = ContextBuilder::new("flappy_bird", "author")
        .window_setup(WindowSetup::default().title("Flappy Bird Game"))
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let state = GameState::new();
    event::run(ctx, event_loop, state)
}
