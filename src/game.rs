use rand::{self, ThreadRng};
use std::f64::consts::PI;
use piston_window::*;
use ::world::*;

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
    /// Turn rate in radians per second
    turn_rate: f64
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
            world: World::new(),
            score: 0,
            actions: Actions::default(),
            timers: Timers::default(),
            rng: rng,
            turn_rate: 0.15 * PI
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
		clear([0.75, 0.75, 0.75, 1.0], g);
        
        self.world.render(c, g);
    }

    pub fn update(&mut self, dt: f64) {
        self.timers.current_time += dt;

        if self.actions.rotate_c_clockwise {
            self.world.ring.rotation -= dt * self.turn_rate;
        }
        if self.actions.rotate_clockwise {
            self.world.ring.rotation += dt * self.turn_rate;
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