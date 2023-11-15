use crate::gameplay::entities::MovingEntity;
use crate::gameplay::gameplay_settings::PLAYER_SPEED;
use crate::gameplay::movement::Velocity;
use crate::gameplay::shooting::Shoot;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::Context;

pub trait Controllable {
    fn apply_controllable_down(&mut self, keyinput: KeyInput);
    fn apply_controllable_up(
        &mut self,
        ctx: &mut Context,
        keyinput: KeyInput,
    ) -> Option<MovingEntity>;
}

impl Controllable for MovingEntity {
    fn apply_controllable_down(&mut self, keyinput: KeyInput) {
        match keyinput.keycode {
            Some(KeyCode::Right) => {
                self.x_axis.set_velocity(PLAYER_SPEED);
            }
            Some(KeyCode::Left) => {
                self.x_axis.set_velocity(-PLAYER_SPEED);
            }
            _ => {}
        }
    }
    fn apply_controllable_up(
        &mut self,
        ctx: &mut Context,
        keyinput: KeyInput,
    ) -> Option<MovingEntity> {
        match keyinput.keycode {
            Some(KeyCode::Right) => {
                self.x_axis.set_velocity(0.0);
            }
            Some(KeyCode::Left) => {
                self.x_axis.set_velocity(0.0);
            }
            // Implement space bar lift of:
            Some(KeyCode::Space) => return Some(self.spawn(ctx)),
            _ => {}
        }
        None
    }
}
