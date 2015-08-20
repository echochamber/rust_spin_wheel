use rand::{self, ThreadRng, Rng};
use piston_window::*;
use game_grid::*;
use super::ring::Ring;

pub struct World {
	pub ring: Ring,
	pub size: f64,
}

impl World {
	pub fn new(size: f64) -> World {
		World {
			ring: Ring::new((size / 2.0, size / 2.0).into(),  100.0),
			size: size
		}
	}

	pub fn center(&self) -> Point {
		return (self.size / 2.0, self.size / 2.0).into();
	}

	pub fn render(&self, c: Context, g: &mut G2d) {

		let border = Rectangle::new_border([0.0, 0.0, 0.0, 1.0], 2.0);

		border.draw(
			[0.0, 0.0, self.size, self.size],
        	&c.draw_state,
        	c.transform,
			g
	    );

		// Ring should be rendered at the center of the world
		let center = self.center();
		self.ring.render(c.trans(center.x, center.y), g);
	}
}