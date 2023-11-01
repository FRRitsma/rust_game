use crate::movement;
use crate::movement::Position;
use ggez;
use ggez::context::Has;
use ggez::graphics::{self, Canvas, Color, DrawParam, Drawable, GraphicsContext, Mesh, Rect};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{glam, Context, GameResult};
use glam::Vec2;
use movement::{CoordinateMovement, Velocity};

pub trait Dimension {
    // Current implementation: xmin, xmax, ymin, ymax
    fn dimension(&self) -> (f32, f32, f32, f32);
}

pub trait Entity {
    fn terminate(&self) -> bool;
    fn position(&self) -> Vec2;
}

pub trait Controllable {
    fn process_control(&mut self);
}

pub struct MovingEntity {
    pub x_axis: CoordinateMovement,
    pub y_axis: CoordinateMovement,
    rectangle: Mesh,
}

impl Drawable for MovingEntity {
    fn draw(&self, canvas: &mut Canvas, _: impl Into<DrawParam>) {
        canvas.draw(&self.rectangle, self.position());
    }
    fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        Some(Rect::new(10.0, 10.0, 10.0, 10.0))
    }
}

impl Entity for MovingEntity {
    fn terminate(&self) -> bool {
        false
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
        }
    }
    pub fn update(&mut self) {
        self.x_axis.update();
        self.y_axis.update();
    }
}

pub struct ControllableMovingEntity {
    pub moving_entity: MovingEntity,
}

impl ControllableMovingEntity {
    pub fn new(ctx: &mut Context, x_axis: CoordinateMovement, y_axis: CoordinateMovement) -> Self {
        ControllableMovingEntity {
            moving_entity: MovingEntity::new(ctx, x_axis, y_axis),
        }
    }

    pub fn apply_controllable_down(&mut self, keyinput: KeyInput) {
        match keyinput.keycode {
            Some(KeyCode::Up) => {
                self.moving_entity.y_axis.set_velocity(-2.0);
            }
            Some(KeyCode::Down) => {
                self.moving_entity.y_axis.set_velocity(2.0);
            }
            Some(KeyCode::Right) => {
                self.moving_entity.x_axis.set_velocity(2.0);
            }
            Some(KeyCode::Left) => {
                self.moving_entity.x_axis.set_velocity(-2.0);
            }
            _ => {}
        }
    }

    pub fn apply_controllable_up(&mut self, keyinput: KeyInput) {
        match keyinput.keycode {
            Some(KeyCode::Up) => {
                self.moving_entity.y_axis.set_velocity(0.0);
            }
            Some(KeyCode::Down) => {
                self.moving_entity.y_axis.set_velocity(0.0);
            }
            Some(KeyCode::Right) => {
                self.moving_entity.x_axis.set_velocity(0.0);
            }
            Some(KeyCode::Left) => {
                self.moving_entity.x_axis.set_velocity(0.0);
            }
            _ => {}
        }
    }
}

#[test]
fn test_moving_object_happy_flow() {
    // let _ = MovingObject::new();
}
