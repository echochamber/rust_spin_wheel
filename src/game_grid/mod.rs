use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {

    pub fn new_offset_polar(origin: &Point, r: f64, theta: f64) -> Point {
        Point {
            x: origin.x + r * theta.cos(),
            y: origin.y + r * theta.sin()
        }
    }

    /// Angle between two points in radians
    pub fn angle_between(&self, p: &Point) -> f64 {
        let m = (self.y - p.y) / (self.x - p.x);

        return if p.x > self.x {
            m.atan()
        } else {
            m.atan() + PI
        }
    }

    pub fn squared_distance_to(&self, p: &Point) -> f64 {
        (self.x - p.x) * (self.x - p.x)
        + (self.y - p.y) * (self.y - p.y)
    }
}

pub struct RectArea {
    pub p1: Point,
    pub p2: Point
}

pub trait Area {
    fn contains(&self, p: &Point) -> bool;
}

impl Area for RectArea {
    fn contains(&self, p: &Point) -> bool {
        (p.x > self.p1.x) ^ (p.x > self.p2.x) &&
        (p.y > self.p1.y) ^ (p.y > self.p2.y)
    }
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

pub trait Moving: Velocity + Position {
    /// dt is the change in time since the position was last updated
    fn move_time(&mut self, dt: f64) {
        *self.x_mut() += self.direction().cos() * self.speed() * dt;
        *self.y_mut() += self.direction().sin() * self.speed() * dt;
    }

    /// Changes the direction of the vector to point to the given target
    fn point_to(&mut self, p: &Point) {
        *self.direction_mut() = self.position().angle_between(p);
    }
}

impl<T> Moving for T where T: Velocity + Position {}

pub trait Turning: Velocity {

    /// Rate of turning in radians/second
    fn turn_rate(&self) -> f64;
    fn turn_rate_mut(&mut self) -> &mut f64;

    /// Turns the object the amount it would turn in dt seconds.
    fn turn_time(&mut self, dt: f64) {
        *self.direction_mut() += dt * self.turn_rate();

        if self.direction() >= 2.0 * PI {
            *self.direction_mut() -= 2.0 * PI;
        }

        if self.direction() < 0.0 {
            *self.direction_mut() += 2.0 * PI;
        }
    }
}