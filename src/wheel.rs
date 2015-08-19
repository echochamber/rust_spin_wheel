
use graphics::types::Color;

pub struct Wheel {
	sections: u32,
	inner_rad: f64,
	outer_rad: f64
}

impl Wheel {
	pub fn new(sections: u32, inner_rad: f64, outer_rad: f64) -> Wheel {
		Wheel {
			sections: sections,
			inner_rad: inner_rad,
			outer_rad: outer_rad
		}
	}
}

pub struct WheelSection {
	color: Color,
	innerArcLength: u32
}