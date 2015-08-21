use graphics::types::Color;

const RED: Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];

#[derive(Debug)]
pub enum GameColors {
    Red,
    Green,
    Blue,
    Yellow
}

impl GameColors {
    pub fn to_color(&self) -> Color {
        match self {
            &GameColors::Red => RED,
            &GameColors::Green => GREEN,
            &GameColors::Blue => BLUE,
            &GameColors::Yellow => YELLOW
        }
    }
}
