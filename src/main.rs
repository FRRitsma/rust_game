#![allow(clippy::unnecessary_wraps)]

use ggez::{event, graphics::Drawable, GameResult};
use my_game::all_states::AllStates;
use my_game::settings::{SPRITE_PATH, WINDOW_HEIGHT, WINDOW_WITH};

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WITH, WINDOW_HEIGHT));
    let (mut ctx, event_loop) = cb.build()?;
    ctx.fs.mount(SPRITE_PATH.as_ref(), true);
    // let state = MenuState::new(&mut ctx);
    let state = AllStates::new(&mut ctx);
    event::run(ctx, event_loop, state);
}
