use crate::entities::MovingEntity;
use crate::movement::Velocity;
use ggez::input::keyboard::{KeyCode, KeyInput};

pub trait Controllable {
    fn apply_controllable_down(&mut self, keyinput: KeyInput);
    fn apply_controllable_up(&mut self, keyinput: KeyInput);
}

impl Controllable for MovingEntity {
    fn apply_controllable_down(&mut self, keyinput: KeyInput) {
        match keyinput.keycode {
            Some(KeyCode::Up) => {
                self.y_axis.set_velocity(-2.0);
            }
            Some(KeyCode::Down) => {
                self.y_axis.set_velocity(2.0);
            }
            Some(KeyCode::Right) => {
                self.x_axis.set_velocity(2.0);
            }
            Some(KeyCode::Left) => {
                self.x_axis.set_velocity(-2.0);
            }
            _ => {}
        }
    }
    fn apply_controllable_up(&mut self, keyinput: KeyInput) {
        match keyinput.keycode {
            Some(KeyCode::Up) => {
                self.y_axis.set_velocity(0.0);
            }
            Some(KeyCode::Down) => {
                self.y_axis.set_velocity(0.0);
            }
            Some(KeyCode::Right) => {
                self.x_axis.set_velocity(0.0);
            }
            Some(KeyCode::Left) => {
                self.x_axis.set_velocity(0.0);
            }
            _ => {}
        }
    }
}
