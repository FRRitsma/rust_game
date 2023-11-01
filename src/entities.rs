use crate::movement;
use crate::movement::Position;
use ggez;
use ggez::context::Has;
use ggez::graphics::{self, Canvas, Color, DrawParam, Drawable, GraphicsContext, Mesh, Rect};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{glam, Context};
use glam::Vec2;
use movement::{CoordinateMovement, Velocity};

pub trait Entity {
    fn terminate(&self) -> bool;
    fn position(&self) -> Vec2;
}

pub struct MovingEntity {
    pub x_axis: CoordinateMovement,
    pub y_axis: CoordinateMovement,
    rectangle: Mesh,
    terminate: bool,
}

impl Drawable for MovingEntity {
    fn draw(&self, canvas: &mut Canvas, _: impl Into<DrawParam>) {
        canvas.draw(&self.rectangle, self.position());
    }
    fn dimensions(&self, _gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        Some(Rect::new(10.0, 10.0, 10.0, 10.0))
    }
}

impl Entity for MovingEntity {
    fn terminate(&self) -> bool {
        self.terminate
    }
    fn position(&self) -> Vec2 {
        Vec2::new(self.x_axis.get_position(), self.y_axis.get_position())
    }
}

impl MovingEntity {
    pub fn new(ctx: &mut Context, x_axis: CoordinateMovement, y_axis: CoordinateMovement) -> Self {
        MovingEntity {
            x_axis,
            y_axis,
            rectangle: Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(10.0, 10.0, 10.0, 10.0),
                Color::WHITE,
            )
            .unwrap(),
            terminate: false,
        }
    }
    pub fn update(&mut self) {
        self.x_axis.update();
        self.y_axis.update();
    }
}

#[test]
fn test_moving_object_happy_flow() {
    // let _ = MovingObject::new();
}
