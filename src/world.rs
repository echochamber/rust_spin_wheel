use std::f64::consts::PI;
use piston_window::*;

const RED: ::graphics::types::Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: ::graphics::types::Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: ::graphics::types::Color = [0.0, 0.0, 1.0, 1.0];
const YELLOW: ::graphics::types::Color = [1.0, 1.0, 0.0, 1.0];

pub struct World {
	pub ring: Ring,
}

pub struct Ring {
	pub rotation: f64
}

impl World {
	pub fn new() -> World {
		World {
			ring: Ring { rotation: 0.0 }
		}
	}

	pub fn render(&self, c: Context, g: &mut G2d) {
		self.ring.render(c, g);
	}
}


impl Ring {
	pub fn new() -> Ring {
		Ring {
			rotation: 0.0
		}
	}

	pub fn render(&self, c: Context, g: &mut G2d) {
		let window_size = 800.0;
    	let ring_size = 200.0;
    	let ring_pos = (window_size - ring_size) / 2.0;
    	let turn_rate = 0.005;
        circle_arc(
        	RED,
        	4.0,
        	PI * (0.0 + self.rotation),
        	PI * (0.5 + self.rotation),
        	[ring_pos, ring_pos, ring_size, ring_size],
			c.transform,
			g
		);

		circle_arc(
        	GREEN,
        	4.0,
        	PI * (0.5 + self.rotation),
        	PI * (1.0 + self.rotation),
        	[ring_pos, ring_pos, ring_size, ring_size],
			c.transform,
			g
		);

		circle_arc(
        	BLUE,
        	4.0,
        	PI * (1.0 + self.rotation),
        	PI * (1.5 + self.rotation),
        	[ring_pos, ring_pos, ring_size, ring_size],
			c.transform,
			g
		);

		circle_arc(
        	YELLOW,
        	4.0,
        	PI * (1.5 + self.rotation),
        	PI * (2.0 + self.rotation),
        	[ring_pos, ring_pos, ring_size, ring_size],
			c.transform,
			g
		);
	}
}