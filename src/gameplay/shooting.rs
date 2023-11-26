use crate::gameplay::entities::{EntityType, MovingEntity};
use crate::gameplay::gameplay_settings::{PROJECTILE_RELOAD_TIME, PROJECTILE_SPEED};
use crate::gameplay::movement::{BoundaryBehavior, CoordinateMovement, Position};
use ggez::Context;
use std::time::Instant;

pub trait Shoot {
    fn spawn(&mut self, ctx: &mut Context) -> Option<MovingEntity>;
}

impl Shoot for MovingEntity {
    fn spawn(&mut self, ctx: &mut Context) -> Option<MovingEntity> {
        // Check reload time:
        if let Some(timer) = self.shoot_timer {
            if timer.elapsed().as_secs() < PROJECTILE_RELOAD_TIME {
                return None;
            } else {
                self.shoot_timer = Some(Instant::now());
            }
        }

        let x_axis: CoordinateMovement = CoordinateMovement::new(
            -1000.0,
            1000.0,
            self.x_axis.get_position(),
            0.0,
            BoundaryBehavior::Die,
        );
        let y_axis: CoordinateMovement = CoordinateMovement::new(
            -1000.0,
            1000.0,
            self.y_axis.get_position(),
            PROJECTILE_SPEED,
            BoundaryBehavior::Die,
        );
        let mut projectile = MovingEntity::new(x_axis, y_axis);
        projectile.set_entity_type(EntityType::Projectile, ctx);
        Some(projectile)
    }
}
