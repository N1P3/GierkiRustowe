use crate::games::game_state::GameState;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, ContextBuilder};

mod games;

fn main() -> ggez::GameResult {
    let (ctx, event_loop) = ContextBuilder::new("GierekPare", "Olek")
        .window_setup(WindowSetup::default().title("Gierki Rustowe Fajne i Kolorowe"))
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let state = GameState::new();
    event::run(ctx, event_loop, state)
}