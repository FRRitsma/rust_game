// Creating scripted behavior for the targets that the player shoots. Targets are MovingEntity

// This function adds multiple targets to the target vector. It is used in the main.rs file. It places moving entities on the same y-axis, spaced across the x-axis.
// The function is implemented using the traits Dimension and Lifetime.

use crate::gameplay::entities::MovingEntity;
use crate::gameplay::movement::{BoundaryBehavior, CoordinateMovement};
use ggez::Context;

pub fn add_targets(ctx: &mut Context, target_vec: &mut Vec<MovingEntity>) {
    let mut x_axis: CoordinateMovement;
    let y_axis: CoordinateMovement =
        CoordinateMovement::new(0.0, 600.0, 50.0, 0.25, BoundaryBehavior::Bounce);
    for i in 0..10 {
        x_axis = CoordinateMovement::new(
            50.0 + 50.0 * i as f32,
            350.0 + 50.0 * i as f32,
            50.0 * i as f32,
            2.0,
            BoundaryBehavior::Bounce,
        );
        let target = MovingEntity::new(ctx, x_axis, y_axis);
        target_vec.push(target);
    }
}
