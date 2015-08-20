use piston_window::*;
use graphics::types::Color;

#[derive(Default)]
pub struct Velocity {
	pub speed: f64, // Units per second
	pub direction: f64 // Radians
}

pub struct Meteorite {
    vector: Vector
}

impl Meteorite {
	pub fn new(pos: Point, velocity: Velocity, color: Color) -> Meteorite {
		Meteorite {
			position: pos,
			velocity: velocity,
			color: color,
			radius: 2.0
		}
	}

	pub fn render(&self, c: Context, g: &mut G2d) {
        Ellipse::new(colors::BLUE).draw(
            [self.position.x - self.radius, self.position.y - self.radius, self.radius * 2.0, self.radius * 2.0],
            &c.draw_state,
            c.transform,
            g
        );
    }

    pub fn move_units(&mut self, dt: f64) {
    	let units = dt * self.velocity.speed;
    	self.position.x += self.velocity.direction.cos() * units;
    	self.position.y += self.velocity.direction.sin() * units;
    }
}