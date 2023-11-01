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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BoundaryBehavior {
    Bounce,
    Wrap,
    Collide,
}

// Implement for structs:
pub struct CoordinateMovement {
    min: f32,
    max: f32,
    position: f32,
    velocity: f32,
    boundary_behavior: BoundaryBehavior,
}

impl Position for CoordinateMovement {
    fn get_min(&self) -> f32 {
        return self.min;
    }
    fn get_max(&self) -> f32 {
        return self.max;
    }
    fn get_position(&self) -> f32 {
        return self.position;
    }
}

impl Velocity for CoordinateMovement {
    fn get_velocity(&self) -> f32 {
        return self.velocity;
    }
    fn set_velocity(&mut self, velocity: f32) {
        self.velocity = velocity;
    }
}

impl Movement for CoordinateMovement {
    fn get_boundary_behavior(&self) -> BoundaryBehavior {
        self.boundary_behavior.clone()
    }
    fn is_at_boundary(&self) -> bool {
        let next_position = self.position + self.velocity;
        return next_position > self.get_max() || next_position < self.get_min();
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
        }
    }
    pub fn update(&mut self) {
        if !self.is_at_boundary() {
            self.position += self.velocity;
            return;
        }
        match self.get_boundary_behavior() {
            BoundaryBehavior::Wrap => {
                let position_offset = self.get_position() - self.get_min();
                let new_position_offset = (position_offset + self.get_velocity())
                    .rem_euclid(self.get_max() - self.get_min());
                self.position = new_position_offset + self.get_min();
            }
            BoundaryBehavior::Bounce => {
                let overshoot = self.position + self.velocity;
                if overshoot > self.get_max() {
                    self.position = 2.0 * self.get_max() - overshoot;
                } else {
                    self.position = 2.0 * self.get_min() - overshoot;
                }
                self.velocity = -self.get_velocity();
            }
            BoundaryBehavior::Collide => {
                let overshoot = self.position + self.velocity;
                if overshoot > self.get_max() {
                    self.position = self.get_max();
                } else {
                    self.position = self.get_min();
                }
                self.velocity = 0.0;
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

//

//
//
// pub enum BoundaryBehavior {
//     Bounce,
//     Wrap,
//     Disappear,
// }
//
// struct Boundaries{
//     x_min: f64,
//     x_max: f64,
//     y_min: f64,
//     y_max: f64,
// }
//
// trait Movement: Position + Velocity {
//
//     fn set_x_position(&mut self, x: f64);
//
//     fn set_y_position(&mut self, x: f64);
//
//     fn set_x_velocity(&mut self, dx: f64);
//
//     fn set_y_velocity(&mut self, dy: f64);
//
//     fn get_boundary_behavior(&self) -> BoundaryBehavior;
//
//     fn move_x(&mut self){
//         if self.is_at_x_boundary(){
//             match self.get_boundary_behavior() {
//                 BoundaryBehavior::Bounce => {
//                     self.set_x_velocity(-self.x_vol());
//                     let new_x = self.x_pos() + self.x_vol();
//                     self.set_x_position(new_x);
//                 },
//                 BoundaryBehavior::Wrap => {
//                     let new_x = (self.x_pos() + self.x_vol()) % 800.0;
//                     self.set_x_position(new_x);
//                 },
//                 BoundaryBehavior::Disappear => {},
//             }
//         }
//         else{
//             let new_x = self.x_pos() + self.x_vol();
//             self.set_x_position(new_x);
//         }
//     }
//
//     fn move_y(&mut self){
//         if self.is_at_y_boundary(){
//             self.set_y_velocity(-self.y_vol());
//         }
//         let new_y = self.y_pos() + self.y_vol();
//         self.set_y_position(new_y);
//     }
//
//         fn is_at_x_boundary(&self) -> bool{
//         // TODO: Fix hardcodes!
//         return self.x_pos() + self.x_vol() > 800.0 || self.x_pos() + self.x_vol() < 0.0
//     }
//
//     fn is_at_y_boundary(&self) -> bool{
//         // TODO: Fix hardcodes!
//         return self.y_pos() + self.y_vol() > 600.0 || self.y_pos() + self.y_vol() < 0.0
//     }
//
//     fn move_object(&mut self) {
//         self.move_x();
//         self.move_y();
//     }
// }
//
//
// pub struct MovingObject {
//     pub x_position: f64,
//     pub y_position: f64,
//     x_velocity: f64,
//     y_velocity: f64,
// }
//
// impl Position for MovingObject {
//     fn x_pos(&self) -> f64 {
//         self.x_position
//     }
//
//     fn y_pos(&self) -> f64 {
//         self.y_position
//     }
// }
//
// impl Velocity for MovingObject {
//     fn x_vol(&self) -> f64 {
//         self.x_velocity
//     }
//
//     fn y_vol(&self) -> f64 {
//         self.y_velocity
//     }
// }
//
// impl Movement for MovingObject {
//
//     fn set_x_position(&mut self, x: f64) {
//         self.x_position = x;
//     }
//
//     fn set_y_position(&mut self, y: f64) {
//         self.y_position = y;
//     }
//
//     fn set_x_velocity(&mut self, dx: f64) {
//         self.x_velocity = dx;
//     }
//
//     fn set_y_velocity(&mut self, dy: f64) {
//         self.y_velocity = dy;
//     }
//
//     fn get_boundary_behavior(&self) -> BoundaryBehavior {
//         BoundaryBehavior::Wrap
//     }
//
// }
//
// impl MovingObject{
//     pub fn new(x_position: f64,
//                y_position: f64,
//                x_velocity: f64,
//                y_velocity: f64,) -> Self{
//         MovingObject {
//             x_position,
//             y_position,
//             x_velocity,
//             y_velocity,
//         }
//     }
//
//     pub fn update(&mut self){
//         self.move_object()
//     }
// }
//
