use rand::{self, ThreadRng, Rng};
use std::f64::consts::PI;
use piston_window::*;
use game_grid::*;
use display::*;
use graphics::types::Color;

pub struct Ring {
	pub position: Point,
	pub radius: f64,
	pub speed: f64,
	pub direction: f64,
	pub turn_rate: f64,
	pub thickness: f64
}

impl Ring {
	pub fn new(position: Point, radius: f64, thickness: f64, turn_rate: f64) -> Ring {
		Ring {
			position: position,
			radius: radius,
			speed: 0.0,
			direction: 0.0,
			turn_rate: turn_rate,
			thickness: thickness
		}
	}

	pub fn render(&self, c: Context, g: &mut G2d) {
        circle_arc(
        	GameColors::Red.into(),
        	self.thickness,
        	PI * 0.0 + self.direction,
        	PI * 0.5 + self.direction,
        	[-self.radius, -self.radius, self.radius * 2.0, self.radius * 2.0],
			c.transform,
			g
		);

		circle_arc(
        	GameColors::Green.into(),
        	self.thickness,
        	PI * 0.5 + self.direction,
        	PI * 1.0 + self.direction,
        	[-self.radius, -self.radius, self.radius * 2.0, self.radius * 2.0],
			c.transform,
			g
		);

		circle_arc(
        	GameColors::Blue.into(),
        	self.thickness,
        	PI * 1.0 + self.direction,
        	PI * 1.5 + self.direction,
        	[-self.radius, -self.radius, self.radius * 2.0, self.radius * 2.0],
			c.transform,
			g
		);

		circle_arc(
        	GameColors::Yellow.into(),
        	self.thickness,
        	PI * 1.5 + self.direction,
        	PI * 2.0 + self.direction,
        	[-self.radius, -self.radius, self.radius * 2.0, self.radius * 2.0],
			c.transform,
			g
		);
	}

	pub fn get_color_at_radian(&self, theta: f64) -> GameColors {
		if is_in_bounds(theta, self.red_bounds()) {
			return GameColors::Red;
		}

		if is_in_bounds(theta, self.green_bounds()) {
			return GameColors::Green;
		}

		if is_in_bounds(theta, self.blue_bounds()) {
			return GameColors::Blue;
		}

		if is_in_bounds(theta, self.yellow_bounds()) {
			return GameColors::Yellow;
		}

		return GameColors::Yellow;
	}

	fn red_bounds(&self) -> (f64, f64) {
		(self.direction + PI * 0.0, self.direction + PI * 0.5)
	}

	fn green_bounds(&self) -> (f64, f64) {
		(self.direction + PI * 0.5, self.direction + PI * 1.0)
	}

	fn blue_bounds(&self) -> (f64, f64) {
		(self.direction + PI * 1.0, self.direction + PI * 1.5)
	}

	fn yellow_bounds(&self) -> (f64, f64) {
		(self.direction + PI * 1.5, self.direction + PI * 0.0)
	}
}

fn is_in_bounds(theta: f64, bounds: (f64, f64)) -> bool {
	if bounds.0 < bounds.1 {
		return theta >= bounds.0 && theta < bounds.1;
	} else {
		return theta >= bounds.0 || theta < bounds.1;
	}
}

derive_turning!(Ring);