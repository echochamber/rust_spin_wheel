use rand::{self, ThreadRng, Rng};
use std::f64::consts::PI;
use piston_window::*;
use meteorite::*;

const RED: ::graphics::types::Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: ::graphics::types::Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: ::graphics::types::Color = [0.0, 0.0, 1.0, 1.0];
const YELLOW: ::graphics::types::Color = [1.0, 1.0, 0.0, 1.0];

pub struct World {
	pub ring: Ring,
	pub size: f64,
	pub size_region: Region,
	pub meteorites: Vec<Meteorite>,
}

pub struct Ring {
	pub rotation: f64,
	pub size: f64
}

impl World {
	pub fn new() -> World {
		World {
			ring: Ring::new(),
			meteorites: Vec::with_capacity(1000),
			size: 700.0,
			size_region: (700.0, 700.0).into()
		}
	}

	pub fn generate_new_meteor(&mut self, rng: &mut ThreadRng) {
		let speed = rng.gen_range(200.0, 400.0) as f64;
		let rand_rad = rng.gen_range(0.0, 2.0 * PI) as f64;

		let offset = (self.size) / 2.0;
        let y = self.size * rand_rad.sin() / 2.0 + offset;
        let x = self.size * rand_rad.cos() / 2.0 + offset;
        let position = Point {x: x, y: y};

        let velocity = Velocity { speed: speed, direction: rand_rad + PI };

        let new_meteor = Meteorite::new(position, velocity, BLUE);
        self.meteorites.push(new_meteor);
	}

	pub fn render(&self, c: Context, g: &mut G2d) {

		let border = Rectangle::new_border([1.0; 4], 2.0);

		border.draw(
			[0.0, 0.0, self.size, self.size],
        	&c.draw_state,
        	c.transform,
			g
	    );

	    for meteorite in &self.meteorites {
            meteorite.render(c, g);
        }
        

		// Ring should be rendered at the center of the world
		let offset = (self.size - self.ring.size) / 2.0;
		self.ring.render(c.trans(offset, offset), g);
	}
}


impl Ring {
	pub fn new() -> Ring {
		Ring {
			rotation: 0.0,
			size: 200.0
		}
	}

	pub fn render(&self, c: Context, g: &mut G2d) {
        circle_arc(
        	RED,
        	4.0,
        	PI * (0.0 + self.rotation),
        	PI * (0.5 + self.rotation),
        	[0.0, 0.0, self.size, self.size],
			c.transform,
			g
		);

		circle_arc(
        	GREEN,
        	4.0,
        	PI * (0.5 + self.rotation),
        	PI * (1.0 + self.rotation),
        	[0.0, 0.0, self.size, self.size],
			c.transform,
			g
		);

		circle_arc(
        	BLUE,
        	4.0,
        	PI * (1.0 + self.rotation),
        	PI * (1.5 + self.rotation),
        	[0.0, 0.0, self.size, self.size],
			c.transform,
			g
		);

		circle_arc(
        	YELLOW,
        	4.0,
        	PI * (1.5 + self.rotation),
        	PI * (2.0 + self.rotation),
        	[0.0, 0.0, self.size, self.size],
			c.transform,
			g
		);
	}
}