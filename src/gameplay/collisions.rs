// TODO: Write implemenation for collisions
// Grid to prevent all vs. all comparison
// Only check moving against moving and moving against static, not static vs. static
use crate::gameplay::entities::{Lifetime, MovingEntity};
use crate::gameplay::movement::Position;
use mockall::automock;

#[automock]
pub trait Dimension {
    fn dimension(&self) -> (f32, f32, f32, f32); // (xmin, xmax, ymin, ymax)
}

impl<T: Dimension> Dimension for &T {
    fn dimension(&self) -> (f32, f32, f32, f32) {
        (*self).dimension()
    }
}

fn collision_left_right(left_min1: f32, right_min2: f32, right_max2: f32) -> bool {
    left_min1 >= right_min2 && left_min1 <= right_max2
}

fn collision_one_dimension(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
    collision_left_right(min1, min2, max2) || collision_left_right(min2, min1, max1)
}

pub fn is_collision<T: Dimension>(object1: T, object2: T) -> bool {
    let (xmin1, xmax1, ymin1, ymax1) = object1.dimension();
    let (xmin2, xmax2, ymin2, ymax2) = object2.dimension();
    collision_one_dimension(xmin1, xmax1, xmin2, xmax2)
        && collision_one_dimension(ymin1, ymax1, ymin2, ymax2)
}

impl Dimension for MovingEntity {
    fn dimension(&self) -> (f32, f32, f32, f32) {
        // TODO: Fix hardcoded dimensions
        let x = self.x_axis.get_position();
        let y = self.y_axis.get_position();
        (
            x,
            x + self.get_sprite_width(),
            y,
            y + self.get_sprite_height(),
        )
    }
}

// This is a function that computes collisions between two vectors, one target and one projectile.
// It applies the set_alive = False method on the collided objects.
// The function is implemented using the traits Dimension and Lifetime
pub fn compute_collisions_target_projectile<T: Dimension + Lifetime>(
    targets: &mut Vec<T>,
    projectiles: &mut Vec<T>,
) {
    for i in 0..targets.len() {
        for j in 0..projectiles.len() {
            if is_collision(&targets[i], &projectiles[j]) {
                targets[i].set_alive(false);
                projectiles[j].set_alive(false);
            }
        }
    }
}

#[test]
fn test_collision_left_right_should_collide() {
    assert!(collision_left_right(5.0, 3.0, 8.0))
}

#[test]
fn test_collision_left_right_should_not_collide() {
    assert!(!collision_left_right(2.0, 3.0, 8.0))
}

#[test]
fn test_collision_one_dimension_should_collide() {
    assert!(collision_one_dimension(0.0, 5.0, 3.0, 10.0));
    assert!(collision_one_dimension(3.0, 10.0, 0.0, 5.0));
}

#[test]
fn test_is_collision_with_overlap() {
    let mut mock1 = MockDimension::new();
    mock1.expect_dimension().returning(|| (0.0, 5.0, 0.0, 5.0));
    let mut mock2 = MockDimension::new();
    mock2.expect_dimension().returning(|| (2.0, 7.0, 2.0, 7.0));
    assert!(is_collision(mock1, mock2));
}

#[test]
fn test_is_collision_no_overlap() {
    let mut mock1 = MockDimension::new();
    mock1.expect_dimension().returning(|| (0.0, 5.0, 0.0, 5.0));
    let mut mock2 = MockDimension::new();
    mock2.expect_dimension().returning(|| (6.0, 7.0, 6.0, 7.0));
    assert!(!is_collision(mock1, mock2));
}

#[test]
fn test_is_collision_strict_inequality() {
    let mut mock1 = MockDimension::new();
    mock1.expect_dimension().returning(|| (0.0, 5.0, 0.0, 5.0));
    let mut mock2 = MockDimension::new();
    mock2.expect_dimension().returning(|| (5.0, 7.0, 5.0, 7.0));
    assert!(is_collision(mock1, mock2))
}
