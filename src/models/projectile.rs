use piston_window::*;
use game_grid::*;
use graphics::types::Color;

pub struct Projectile {
	pub position: Point,
	pub radius: f64,
	pub speed: f64,
	pub direction: f64,
	pub color: Color
}

impl Projectile {
	pub fn new(position: Point, direction: f64, speed: f64, radius: f64, color: Color) -> Projectile {
		Projectile {
			position: position,
			speed: speed,
			direction: direction,
			radius: radius,
			color: color
		}
	}

	pub fn render(&self, c: Context, g: &mut G2d) {
		Ellipse::new(self.color)
			.draw(
            	[
            		self.position.x - self.radius,
            		self.position.y - self.radius,
            		self.radius * 2.0,
            		self.radius * 2.0
        		],
            	&c.draw_state,
            	c.transform,
            	g
        	);
	}
}

derive_velocity!(Projectile);