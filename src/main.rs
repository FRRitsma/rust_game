//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

mod entities;
use entities::MovingObject;

struct MainState {
    moving_object: MovingObject,
    circle: graphics::Mesh,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            100.0,
            2.0,
            Color::WHITE,
        )?;

        Ok(MainState {moving_object: MovingObject::new(0.0, 0.0, 5.0, 2.0), circle })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.moving_object.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // canvas.draw(&self.circle, Vec2::new(self.pos_x, 380.0));
        canvas.draw(&self.circle, Vec2::new(self.moving_object.x_position as f32, self.moving_object.y_position as f32));


        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keyinput: KeyInput, _repeat: bool) -> GameResult {
        match keyinput.keycode {
            Some(KeyCode::Up) => {
                Ok(())
            }
            _ => Ok(()), // Do nothing for other keys
        }
    }



}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}