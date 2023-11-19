use crate::gameplay::entities::{EntityType, MovingEntity};
use crate::gameplay::gameplay_settings::PROJECTILE_SPEED;
use crate::gameplay::movement::{BoundaryBehavior, CoordinateMovement, Position};
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
        let mut projectile = MovingEntity::new(x_axis, y_axis);
        projectile.set_entity_type(EntityType::Projectile, ctx);
        projectile
    }
}
