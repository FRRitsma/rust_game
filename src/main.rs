#![allow(clippy::unnecessary_wraps)]

use ggez::{event, GameResult};
use my_game::all_states::AllStates;
use my_game::settings::{SPRITE_PATH, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn main() -> GameResult {
    let mut window_mode = ggez::conf::WindowMode::default();
    window_mode.width = WINDOW_WIDTH;
    window_mode.height = WINDOW_HEIGHT;
    window_mode.resize_on_scale_factor_change = true;
    window_mode.resizable = true;

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("my_game", "my_author")
        .window_mode(window_mode)
        .build()
        .expect("Failed to build context");

    ctx.fs.mount(SPRITE_PATH.as_ref(), true);
    let state = AllStates::new(&mut ctx);
    event::run(ctx, event_loop, state);
}
