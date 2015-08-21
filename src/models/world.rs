use rand::{self, ThreadRng, Rng};
use piston_window::*;
use game_grid::*;
use super::ring::Ring;
use super::projectile::Projectile;

pub struct World {
	pub ring: Ring,
	pub projectiles: Vec<Projectile>,
	pub size: f64,
}

impl World {
	pub fn new(size: f64, ring: Ring) -> World {
		World {
			ring: ring,
			projectiles: Vec::new(),
			size: size
		}
	}

	pub fn center(&self) -> Point {
		return (self.size / 2.0, self.size / 2.0).into();
	}

	pub fn render(&self, c: Context, g: &mut G2d) {

		// Ring should be rendered at the center of the world
		let center = self.center();
		self.ring.render(c.trans(center.x, center.y), g);

		for projectiles in &self.projectiles {
            projectiles.render(c, g);
        }
	}

	pub fn update(&mut self, dt: f64) {
		for projectiles in &mut self.projectiles {
            projectiles.move_time(dt);
        }
	}
}