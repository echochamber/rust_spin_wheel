#[macro_export]
macro_rules! derive_position {
    ($t:ty) => {
        impl ::game_grid::Position for $t {
            fn x(&self) -> f64 { self.position.x }
            fn x_mut(&mut self) -> &mut f64 { &mut self.position.x }
            fn y(&self) -> f64 { self.position.y }
            fn y_mut(&mut self) -> &mut f64 { &mut self.position.y }
        }
    }
}

#[macro_export]
macro_rules! derive_velocity {
    ($t:ty) => {
        impl ::game_grid::Position for $t {
            fn x(&self) -> f64 { self.position.x }
            fn x_mut(&mut self) -> &mut f64 { &mut self.position.x }
            fn y(&self) -> f64 { self.position.y }
            fn y_mut(&mut self) -> &mut f64 { &mut self.position.y }
        }

        impl ::game_grid::Velocity for $t {
            fn speed(&self) -> f64 { self.speed }
            fn speed_mut(&mut self) -> &mut f64 { &mut self.speed }
            fn direction(&self) -> f64 { self.direction }
            fn direction_mut(&mut self) -> &mut f64 { &mut self.direction }
        }
    }
}

#[macro_export]
macro_rules! derive_turning {
    ($t:ty) => {
        impl ::game_grid::Position for $t {
            fn x(&self) -> f64 { self.position.x }
            fn x_mut(&mut self) -> &mut f64 { &mut self.position.x }
            fn y(&self) -> f64 { self.position.y }
            fn y_mut(&mut self) -> &mut f64 { &mut self.position.y }
        }

        impl ::game_grid::Velocity for $t {
            fn speed(&self) -> f64 { self.speed }
            fn speed_mut(&mut self) -> &mut f64 { &mut self.speed }
            fn direction(&self) -> f64 { self.direction }
            fn direction_mut(&mut self) -> &mut f64 { &mut self.direction }
        }

        impl ::game_grid::Turning for $t {
            fn turn_rate(&self) -> f64 { self.turn_rate }
            fn turn_rate_mut(&mut self) -> &mut f64 { &mut self.turn_rate }
        }
    }
}