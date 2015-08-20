use std::f64::consts::PI;

pub struct Point {
    pub x: f64,
    pub y: f64
}

// (x, y) coordinates
impl Into<Point> for (f64, f64) {
    fn into(self) -> Point {
        match self {
            (x, y) => Point {
                x: x,
                y: y
            }
        }
    }
}


pub trait Position {
    /// Returns the x coordinate of the object
    fn x(&self) -> f64;

    /// Returns a mutable reference to the x coordinate
    fn x_mut(&mut self) -> &mut f64;

    /// Returns the y coordinate of the object
    fn y(&self) -> f64;

    /// Returns a mutable reference to the y coordinate
    fn y_mut(&mut self) -> &mut f64;

    /// Returns the position of the object
    fn position(&self) -> Point {
        (self.x(), self.y()).into()
    }
}

pub trait Velocity {
    // speed in units/second
    fn speed(&self) -> f64;
    fn speed_mut(&mut self) -> &mut f64;

    // Direction in radians
    fn direction(&self) -> f64;
    fn direction_mut(&mut self) -> &mut f64;

    
}

pub trait Moving: Position + Velocity {
    /// dt is the change in time since the position was last updated
    fn move_time(&mut self, dt: f64) {
        *self.x_mut() += self.direction().cos() * self.speed();
        *self.y_mut() += self.direction().sin() * self.speed();
    }

    /// Changes the direction of the vector to point to the given target
    fn point_to<T: Position>(&mut self, target: T) {
        let m = (self.y() - target.y()) / (self.x() - target.x());

        *self.direction_mut() = if target.x() > self.x() {
            m.atan()
        } else {
            m.atan() + PI
        };
    }
}

pub trait Turning: Position {

    // Rate of turning in radians/second
    fn turn_rate(&self) -> f64;

    fn turn_rate_mut(&mut self) -> &mut f64;

    fn turn_time(&mut self, dt: f64) {
        *self.turn_rate_mut() += dt * self.turn_rate()
    }
}