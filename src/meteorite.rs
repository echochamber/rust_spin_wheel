use ::graphics::types::Color;
use piston_window::*;

const RED: ::graphics::types::Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: ::graphics::types::Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: ::graphics::types::Color = [0.0, 0.0, 1.0, 1.0];
const YELLOW: ::graphics::types::Color = [1.0, 1.0, 0.0, 1.0];

#[derive(Default)]
pub struct Velocity {
	pub speed: f64, // Units per second
	pub direction: f64 // Radians
}

#[derive(Default)]
pub struct Position {
	pub x: f64,
	pub y: f64
}

pub struct Meteorite {
	velocity: Velocity,
	position: Position,
	color: Color,
	radius: f64
}

impl Meteorite {
	pub fn new(pos: Position, color: Color) -> Meteorite {
		Meteorite {
			position: pos,
			velocity: Velocity::default(),
			color: color,
			radius: 2.0
		}
	}

	pub fn render(&self, c: Context, g: &mut G2d) {
        Ellipse::new(BLUE).draw(
            [self.position.x - self.radius, self.position.y - self.radius, self.radius * 2.0, self.radius * 2.0],
            &c.draw_state,
            c.transform,
            g
        );
    }
}