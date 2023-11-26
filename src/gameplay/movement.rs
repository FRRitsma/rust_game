use crate::gameplay::entities::Lifetime;

pub trait Position {
    fn get_min(&self) -> f32;
    fn get_max(&self) -> f32;
    fn get_position(&self) -> f32;
}

pub trait Velocity {
    fn get_velocity(&self) -> f32;
    fn set_velocity(&mut self, velocity: f32);
}

trait Movement: Velocity + Position {
    fn get_boundary_behavior(&self) -> BoundaryBehavior;
    fn is_at_boundary(&self) -> bool;
}

pub trait Update {
    fn update(&mut self);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BoundaryBehavior {
    Bounce,
    Wrap,
    Collide,
    Die,
}

// Implement for structs:
#[derive(Clone, Copy)]
pub struct CoordinateMovement {
    min: f32,
    max: f32,
    position: f32,
    velocity: f32,
    boundary_behavior: BoundaryBehavior,
    is_alive: bool,
}

impl Position for CoordinateMovement {
    fn get_min(&self) -> f32 {
        self.min
    }
    fn get_max(&self) -> f32 {
        self.max
    }
    fn get_position(&self) -> f32 {
        self.position
    }
}

impl Velocity for CoordinateMovement {
    fn get_velocity(&self) -> f32 {
        self.velocity
    }
    fn set_velocity(&mut self, velocity: f32) {
        self.velocity = velocity;
    }
}

impl Movement for CoordinateMovement {
    fn get_boundary_behavior(&self) -> BoundaryBehavior {
        self.boundary_behavior
    }
    fn is_at_boundary(&self) -> bool {
        let next_position = self.position + self.velocity;
        next_position > self.get_max() || next_position < self.get_min()
    }
}

impl Lifetime for CoordinateMovement {
    fn is_alive(&self) -> bool {
        self.is_alive
    }
    fn set_alive(&mut self, alive: bool) {
        self.is_alive = alive;
    }
}

// Implement for self:
impl CoordinateMovement {
    pub fn new(
        min: f32,
        max: f32,
        position: f32,
        velocity: f32,
        boundary_behavior: BoundaryBehavior,
    ) -> Self {
        assert!(min < max);
        CoordinateMovement {
            min,
            max,
            position,
            velocity,
            boundary_behavior,
            is_alive: true,
        }
    }

    fn boundary_behavior_die(&mut self) {
        self.is_alive = false;
        self.set_velocity(0.0);
    }

    fn boundary_behavior_collide(&mut self) {
        let overshoot = self.position + self.velocity;
        if overshoot > self.get_max() {
            self.position = self.get_max();
        } else {
            self.position = self.get_min();
        }
        self.velocity = 0.0;
    }

    fn boundary_behavior_bounce(&mut self) {
        let overshoot = self.position + self.velocity;
        if overshoot > self.get_max() {
            self.position = 2.0 * self.get_max() - overshoot;
        } else {
            self.position = 2.0 * self.get_min() - overshoot;
        }
        self.velocity = -self.get_velocity();
    }

    fn boundary_behavior_wrap(&mut self) {
        let position_offset = self.get_position() - self.get_min();
        let new_position_offset =
            (position_offset + self.get_velocity()).rem_euclid(self.get_max() - self.get_min());
        self.position = new_position_offset + self.get_min();
    }
}

impl Update for CoordinateMovement {
    fn update(&mut self) {
        if !self.is_at_boundary() {
            self.position += self.velocity;
            return;
        }
        match self.get_boundary_behavior() {
            BoundaryBehavior::Wrap => {
                self.boundary_behavior_wrap();
            }
            BoundaryBehavior::Bounce => {
                self.boundary_behavior_bounce();
            }
            BoundaryBehavior::Collide => {
                self.boundary_behavior_collide();
            }
            BoundaryBehavior::Die => {
                self.boundary_behavior_die();
            }
        }
    }
}

// Tests:
#[test]
#[should_panic]
fn test_invalid_min_max() {
    let _ = CoordinateMovement::new(1.0, 1.0, 1.0, 2.0, BoundaryBehavior::Wrap);
}

#[test]
fn test_valid_min_max() {
    let _ = CoordinateMovement::new(1.0, 3.0, 1.0, 2.0, BoundaryBehavior::Wrap);
}

#[test]
fn test_update() {
    let mut coordinate_movement =
        CoordinateMovement::new(1.0, 3.0, 1.0, 2.0, BoundaryBehavior::Wrap);
    coordinate_movement.update();
    assert_eq!(coordinate_movement.get_position(), 3.0);
}

#[test]
fn test_is_at_boundary() {
    let movement_object = CoordinateMovement::new(0.0, 10.0, 7.0, 5.0, BoundaryBehavior::Wrap);
    assert!(movement_object.is_at_boundary());
    let movement_object = CoordinateMovement::new(0.0, 10.0, 7.0, 2.0, BoundaryBehavior::Wrap);
    assert!(!movement_object.is_at_boundary());
}

#[test]
fn test_wraps_through_boundary_happy_flow() {
    let mut movement_object = CoordinateMovement::new(0.0, 10.0, 7.0, 5.0, BoundaryBehavior::Wrap);
    movement_object.update();
    assert_eq!(movement_object.get_position(), 2.0);
}

#[test]
fn test_wraps_through_boundary_offset_positive() {
    let mut movement_object = CoordinateMovement::new(1.0, 10.0, 7.0, 4.0, BoundaryBehavior::Wrap);
    movement_object.update();
    assert_eq!(movement_object.get_position(), 2.0);
}

#[test]
fn test_wraps_through_boundary_offset_negative() {
    let mut movement_object = CoordinateMovement::new(1.0, 10.0, 3.0, -4.0, BoundaryBehavior::Wrap);
    movement_object.update();
    assert_eq!(movement_object.get_position(), 8.0);
}

#[test]
fn test_bounces_at_zero_negative_velocity() {
    let mut movement_object =
        CoordinateMovement::new(0.0, 10.0, 3.0, -4.0, BoundaryBehavior::Bounce);
    movement_object.update();
    assert_eq!(movement_object.get_position(), 1.0);
    movement_object.update();
    assert_eq!(movement_object.get_position(), 5.0);
}

#[test]
fn test_bounces_at_offset_negative_velocity() {
    let mut movement_object =
        CoordinateMovement::new(1.0, 10.0, 3.0, -4.0, BoundaryBehavior::Bounce);
    movement_object.update();
    assert_eq!(movement_object.get_position(), 3.0);
    movement_object.update();
    assert_eq!(movement_object.get_position(), 7.0);
}

#[test]
fn test_bounces_at_positive_velocity() {
    let mut movement_object =
        CoordinateMovement::new(1.0, 10.0, 7.0, 4.0, BoundaryBehavior::Bounce);
    movement_object.update();
    assert_eq!(movement_object.get_position(), 9.0);
    movement_object.update();
    assert_eq!(movement_object.get_position(), 5.0);
}
