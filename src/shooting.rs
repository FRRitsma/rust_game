use crate::entities::MovingEntity;
use crate::movement::{BoundaryBehavior, CoordinateMovement, Position};
use crate::settings::PROJECTILE_SPEED;
use ggez::Context;

pub trait Shoot {
    fn spawn(&self, ctx: &mut Context) -> MovingEntity;
}

impl Shoot for MovingEntity {
    fn spawn(&self, ctx: &mut Context) -> MovingEntity {
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
        MovingEntity::new(ctx, x_axis, y_axis)
    }
}
