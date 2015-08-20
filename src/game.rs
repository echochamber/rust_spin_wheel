use rand::{self, ThreadRng, Rng};
use std::f64::consts::PI;
use piston_window::*;
use models::*;
use game_grid::*;

const RED: ::graphics::types::Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: ::graphics::types::Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: ::graphics::types::Color = [0.0, 0.0, 1.0, 1.0];
const YELLOW: ::graphics::types::Color = [1.0, 1.0, 0.0, 1.0];


/// The data structure that drives the game
pub struct Game {
    /// The world contains everything that needs to be drawn
    world: World,
    /// The current score of the player
    score: u32,
    /// The active actions
    actions: Actions,
    /// Timers needed by the game
    timers: Timers,
    /// A random number generator
    rng: ThreadRng,
    /// The size of the games viewport
    size: f64
}

/// Active actions (toggled by user input)
#[derive(Default)]
struct Actions {
    rotate_clockwise: bool,
    rotate_c_clockwise: bool
}

/// Timers to handle creation of bullets, enemies and particles
#[derive(Default)]
struct Timers {
    current_time: f64,
    last_spawned_projectile: f64
}

impl Game {
	pub fn new() -> Game {
        let mut rng = rand::thread_rng();
        Game {
            world: World::new(700.0),
            score: 0,
            actions: Actions::default(),
            timers: Timers::default(),
            rng: rng,
            size: 800.0
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
		clear([0.75, 0.75, 0.75, 1.0], g);


		// Horzontally center the world, but vertically align it with the bottom of the game viewport.
		let offset = self.size - self.world.size;
        self.world.render(c.trans(offset / 2.0, offset), g);
    }

    pub fn world_origin(&self) -> (f64, f64) {
    	let offset = self.size - self.world.size;
    	((offset / 2.0) - (self.world.size / 2.0), offset - (self.world.size / 2.0))
    }

    pub fn update(&mut self, dt: f64) {
        self.timers.current_time += dt;

        if self.actions.rotate_c_clockwise {
            self.world.ring.turn_time(-dt);
        }
        if self.actions.rotate_clockwise {
            self.world.ring.turn_time(dt);
        }
    }

	/// Processes a key press
    pub fn key_press(&mut self, key: Key) {
        self.handle_key(key, true);
    }

    /// Processes a key release
    pub fn key_release(&mut self, key: Key) {
        self.handle_key(key, false);
    }

    /// Handles a key press or release
    fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::Up => self.actions.rotate_clockwise = pressed,
            Key::Down => self.actions.rotate_c_clockwise = pressed,
            _ => ()
        }
    }
}