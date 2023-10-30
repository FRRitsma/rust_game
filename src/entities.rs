use crate::movement;
use movement::CoordinateMovement;
use ggez;
use ggez::{Context, GameResult, glam};
use ggez::graphics::{self, Color, Drawable, Canvas, DrawParam, GraphicsContext, Rect, Mesh};
use ggez::context::Has;
use glam::{Vec2};
use crate::movement::Position;

pub trait Entity{
    fn is_alive(&self) -> bool;
    fn position(&self) -> Vec2;
}

pub struct MovingEntity{
    pub x_axis: CoordinateMovement,
    pub y_axis: CoordinateMovement,
    rectangle: Mesh,
}

impl Drawable for MovingEntity{

    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>){
        canvas.draw(&self.rectangle, param);
    }

    fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        Some(Rect::new(10.0, 10.0, 10.0, 10.0))
    }
}

impl Entity for MovingEntity{
    fn is_alive(&self) -> bool {
        true
    }
    fn position(&self) -> Vec2 {
        Vec2::new(self.x_axis.get_position(), self.y_axis.get_position())
    }
}

impl MovingEntity{
    pub fn new(ctx: &mut Context,
        x_axis: CoordinateMovement, y_axis: CoordinateMovement
    ) -> Self{
        MovingEntity{
            x_axis,
            y_axis,
            rectangle: graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(10.0, 10.0, 10.0, 10.0),
            Color::WHITE,
        ).unwrap()
        }
    }
    pub fn update(&mut self){
        self.x_axis.update();
        self.y_axis.update();
    }
}

#[test]
fn test_moving_object_happy_flow(){
    // let _ = MovingObject::new();
}