use crate::movement;
use movement::CoordinateMovement;
use movement::BoundaryBehavior;
use ggez;
use ggez::{event, glam};
use ggez::graphics::{self, Canvas, DrawParam, Image, Color};
use ggez::Context;
use ggez::GameResult;
use glam::{Vec2, vec2};
use crate::movement::Position;

pub trait Entity{
    fn is_alive(&self) -> bool;
    fn position(&self) -> Vec2;
    fn draw(&self, ctx: &mut Context) -> GameResult;
}

pub struct MovingEntity{
    pub x_axis: CoordinateMovement,
    pub y_axis: CoordinateMovement,
}

impl Entity for MovingEntity{
    fn is_alive(&self) -> bool {
        true
    }
    fn position(&self) -> Vec2 {
        Vec2::new(self.x_axis.get_position(), self.y_axis.get_position())
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        // // Set the draw color to the entity's color
        // graphics::set_color(ctx, self.color)?;
        //
        // // Calculate the position for the circle
        // let circle_center = self.position + na::Vector2::new(self.radius, self.radius);
        //
        // // Draw a filled circle
        // let circle = graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     circle_center,
        //     self.radius,
        //     2.0, // Line width (ignored for fill)
        //     self.color,
        // )?;
        //
        // graphics::draw(ctx, &circle, DrawParam::new())?;

        Ok(())
    }
}

impl MovingEntity{
    pub fn new(
        x_axis: CoordinateMovement, y_axis: CoordinateMovement
    ) -> Self{
        MovingEntity{
            x_axis,
            y_axis,
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