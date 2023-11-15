use crate::gameplay::movement::{Position, Update};
use ggez;

use ggez::graphics::Image;

use crate::gameplay::movement::CoordinateMovement;
use ggez::{glam, Context};
use glam::Vec2;

pub trait Entity {
    fn position(&self) -> Vec2;
}

pub trait Lifetime {
    fn is_alive(&self) -> bool;
    fn set_alive(&mut self, alive: bool);
}

pub struct MovingEntity {
    pub x_axis: CoordinateMovement,
    pub y_axis: CoordinateMovement,
    sprite: Image,
    is_alive: bool,
}

impl Lifetime for MovingEntity {
    fn is_alive(&self) -> bool {
        // Both axes and the object itself must still be alive:
        self.x_axis.is_alive() && self.y_axis.is_alive() && self.is_alive
    }
    fn set_alive(&mut self, alive: bool) {
        self.is_alive = alive;
    }
}

impl Entity for MovingEntity {
    fn position(&self) -> Vec2 {
        Vec2::new(self.x_axis.get_position(), self.y_axis.get_position())
    }
}

impl MovingEntity {
    pub fn new(ctx: &mut Context, x_axis: CoordinateMovement, y_axis: CoordinateMovement) -> Self {
        MovingEntity {
            x_axis,
            y_axis,
            sprite: Image::from_path(ctx, "/test.png").unwrap(),
            is_alive: true,
        }
    }

    pub fn get_sprite(&self) -> &Image {
        &self.sprite
    }
}

impl Update for MovingEntity {
    fn update(&mut self) {
        self.x_axis.update();
        self.y_axis.update();
    }
}

impl Update for Vec<MovingEntity> {
    fn update(&mut self) {
        for moving_entity in self.iter_mut() {
            moving_entity.update();
        }
    }
}
