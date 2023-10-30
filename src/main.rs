#![allow(clippy::unnecessary_wraps)]

use ggez::{
    event,
    glam::*,
    graphics::{self, Color, Drawable},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};
use ggez::graphics::DrawParam;

use my_game::entities::{MovingEntity, Entity};
use my_game::movement::{BoundaryBehavior, CoordinateMovement};

struct MainState {
    moving_entity: MovingEntity,
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
        let x_axis: CoordinateMovement = CoordinateMovement::new(0.0,800.0, 0.0, 3.0, BoundaryBehavior::Bounce);
        let y_axis: CoordinateMovement = CoordinateMovement::new(0.0,600.0, 0.0, 2.0, BoundaryBehavior::Bounce);
        let moving_entity: MovingEntity = MovingEntity::new(ctx, x_axis, y_axis);
        Ok(MainState { moving_entity, circle })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.moving_entity.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // canvas.draw(&self.circle, self.moving_entity.position());
        self.moving_entity.draw(&mut canvas, DrawParam::default());
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