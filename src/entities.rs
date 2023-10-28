trait Position {
    fn x_pos(&self) -> f64;
    fn y_pos(&self) -> f64;
}

trait Velocity {
    fn x_vol(&self) -> f64;
    fn y_vol(&self) -> f64;
}

pub enum BoundaryBehavior {
    Bounce,
    Wrap,
    Disappear,
}

trait Movement: Position + Velocity {

    fn set_x_position(&mut self, x: f64);

    fn set_y_position(&mut self, x: f64);

    fn set_x_velocity(&mut self, dx: f64);

    fn set_y_velocity(&mut self, dy: f64);

    fn get_boundary_behavior(&self) -> BoundaryBehavior;

    fn move_x(&mut self){
        if self.is_at_x_boundary(){
            match self.get_boundary_behavior() {
                BoundaryBehavior::Bounce => {
                    self.set_x_velocity(-self.x_vol());
                    let new_x = self.x_pos() + self.x_vol();
                    self.set_x_position(new_x);
                },
                BoundaryBehavior::Wrap => {
                    let new_x = (self.x_pos() + self.x_vol()) % 800.0;
                    self.set_x_position(new_x);
                },
                BoundaryBehavior::Disappear => {},
            }
        }
        else{
            let new_x = self.x_pos() + self.x_vol();
            self.set_x_position(new_x);
        }
    }

    fn move_y(&mut self){
        if self.is_at_y_boundary(){
            self.set_y_velocity(-self.y_vol());
        }
        let new_y = self.y_pos() + self.y_vol();
        self.set_y_position(new_y);
    }

        fn is_at_x_boundary(&self) -> bool{
        // TODO: Fix hardcodes!
        return self.x_pos() + self.x_vol() > 800.0 || self.x_pos() + self.x_vol() < 0.0
    }

    fn is_at_y_boundary(&self) -> bool{
        // TODO: Fix hardcodes!
        return self.y_pos() + self.y_vol() > 600.0 || self.y_pos() + self.y_vol() < 0.0
    }

    fn move_object(&mut self) {
        self.move_x();
        self.move_y();
    }
}


pub struct MovingObject {
    pub x_position: f64,
    pub y_position: f64,
    x_velocity: f64,
    y_velocity: f64,
}

impl Position for MovingObject {
    fn x_pos(&self) -> f64 {
        self.x_position
    }

    fn y_pos(&self) -> f64 {
        self.y_position
    }
}

impl Velocity for MovingObject {
    fn x_vol(&self) -> f64 {
        self.x_velocity
    }

    fn y_vol(&self) -> f64 {
        self.y_velocity
    }
}

impl Movement for MovingObject {

    fn set_x_position(&mut self, x: f64) {
        self.x_position = x;
    }

    fn set_y_position(&mut self, y: f64) {
        self.y_position = y;
    }

    fn set_x_velocity(&mut self, dx: f64) {
        self.x_velocity = dx;
    }

    fn set_y_velocity(&mut self, dy: f64) {
        self.y_velocity = dy;
    }

    fn get_boundary_behavior(&self) -> BoundaryBehavior {
        BoundaryBehavior::Wrap
    }

}

impl MovingObject{
    pub fn new(x_position: f64,
               y_position: f64,
               x_velocity: f64,
               y_velocity: f64,) -> Self{
        MovingObject {
            x_position,
            y_position,
            x_velocity,
            y_velocity,
        }
    }

    pub fn update(&mut self){
        self.move_object()
    }
}

