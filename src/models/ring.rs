use rand::{self, ThreadRng, Rng};
use std::f64::consts::PI;
use piston_window::*;
use game_grid::*;

const RED: ::graphics::types::Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: ::graphics::types::Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: ::graphics::types::Color = [0.0, 0.0, 1.0, 1.0];
const YELLOW: ::graphics::types::Color = [1.0, 1.0, 0.0, 1.0];

pub struct Ring {
	pub position: Point,
	pub radius: f64,
	pub speed: f64,
	pub direction: f64,
	pub turn_rate: f64
}

impl Ring {
	pub fn new(position: Point, radius: f64) -> Ring {
		Ring {
			position: position,
			radius: radius,
			speed: 0.0,
			direction: 0.0,
			turn_rate: 0.25 * PI
		}
	}

	pub fn render(&self, c: Context, g: &mut G2d) {
        circle_arc(
        	RED,
        	4.0,
        	PI * (0.0 + self.direction),
        	PI * (0.5 + self.direction),
        	[-self.radius, -self.radius, self.radius * 2.0, self.radius * 2.0],
			c.transform,
			g
		);

		circle_arc(
        	GREEN,
        	4.0,
        	PI * (0.5 + self.direction),
        	PI * (1.0 + self.direction),
        	[-self.radius, -self.radius, self.radius * 2.0, self.radius * 2.0],
			c.transform,
			g
		);

		circle_arc(
        	BLUE,
        	4.0,
        	PI * (1.0 + self.direction),
        	PI * (1.5 + self.direction),
        	[-self.radius, -self.radius, self.radius * 2.0, self.radius * 2.0],
			c.transform,
			g
		);

		circle_arc(
        	YELLOW,
        	4.0,
        	PI * (1.5 + self.direction),
        	PI * (2.0 + self.direction),
        	[-self.radius, -self.radius, self.radius * 2.0, self.radius * 2.0],
			c.transform,
			g
		);
	}
}

derive_turning!(Ring);