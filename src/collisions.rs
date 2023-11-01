// TODO: Write implemenation for collisions
// What must be known? Position, Speed, Width, Height
// Grid to prevent all vs. all comparison
// Only check moving against moving and moving against static, not static vs. static

// First: all against all comparison

// How to check overlap:
// dim: obj1_min > obj2_min and obj1_min < obj2_max

pub trait Dimension {
    fn dimension(&self) -> (f32, f32, f32, f32);
}

fn collision_left_right(left_min1: f32, right_min2: f32, right_max2: f32) -> bool {
    // Must be used twice to check whether an object overlaps in one dimension
    left_min1 > right_min2 && left_min1 < right_max2
}

fn collision_one_dimension(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
    // Returns true if two objects overlap in one dimension
    collision_left_right(min1, min2, max2) || collision_left_right(min2, min1, max1)
}

pub fn is_collision<T: Dimension>(object1: T, object2: T) -> bool {
    // Returns true if two objets overlap in both dimensions
    let (xmin1, xmax1, ymin1, ymax1) = object1.dimension();
    let (xmin2, xmax2, ymin2, ymax2) = object2.dimension();
    collision_one_dimension(xmin1, xmax1, xmin2, xmax2)
        && collision_one_dimension(ymin1, ymax1, ymin2, ymax2)
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
    assert!(collision_one_dimension(0.0, 5.0, 3.0, 10.0))
}

#[test]
fn test_collision_one_dimension_should_collide_symmetrical() {
    assert!(collision_one_dimension(3.0, 10.0, 0.0, 5.0))
}
