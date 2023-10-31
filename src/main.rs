#![allow(clippy::unnecessary_wraps)]

use ggez::graphics::DrawParam;
use ggez::{
    event,
    glam::*,
    graphics::{self, Drawable},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

use my_game::entities::{ControllableMovingEntity, MovingEntity};
use my_game::movement::{BoundaryBehavior, CoordinateMovement};

struct MainState {
    entity_vec: Vec<ControllableMovingEntity>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut entity_vec: Vec<ControllableMovingEntity> = Vec::new();
        let x_axis: CoordinateMovement =
            CoordinateMovement::new(0.0, 800.0, 0.0, 3.0, BoundaryBehavior::Bounce);
        let y_axis: CoordinateMovement =
            CoordinateMovement::new(0.0, 600.0, 0.0, 2.0, BoundaryBehavior::Bounce);
        entity_vec.push(ControllableMovingEntity::new(ctx, x_axis, y_axis));
        let x_axis: CoordinateMovement =
            CoordinateMovement::new(0.0, 800.0, 0.0, -5.0, BoundaryBehavior::Collide);
        let y_axis: CoordinateMovement =
            CoordinateMovement::new(0.0, 600.0, 0.0, 3.0, BoundaryBehavior::Collide);
        entity_vec.push(ControllableMovingEntity::new(ctx, x_axis, y_axis));

        Ok(MainState { entity_vec })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Apply updates to entities:
        for moving_entity in self.entity_vec.iter_mut() {
            moving_entity.moving_entity.update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for moving_entity in &self.entity_vec {
            moving_entity
                .moving_entity
                .draw(&mut canvas, DrawParam::default());
        }
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keyinput: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        self.entity_vec[0].apply_controllable(keyinput);
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
