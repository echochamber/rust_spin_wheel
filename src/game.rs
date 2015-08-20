use rand::{self, ThreadRng, Rng};
use std::f64::consts::PI;
use piston_window::*;
use world::*;
use meteorite::*;

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
    /// Turn rate in radians per second
    turn_rate: f64,

    /// The size of the games viewport
    size: f64,

    /// The region it occupies
    size_region: Region
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
            turn_rate: 0.15 * PI,
            size: 800.0,
            size_region: (800.0, 800.0).into()
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
            self.world.ring.rotation -= dt * self.turn_rate;
        }
        if self.actions.rotate_clockwise {
            self.world.ring.rotation += dt * self.turn_rate;
        }

        // Move enemies in the player's direction
        for meteorite in &mut self.world.meteorites {
            meteorite.move_units(dt);
        }

        // Remove bullets outside the viewport
        { // Shorten the lifetime of size
	        let size = &self.world.size_region;
	        self.world.meteorites.retain(|b| size.contains(b.position));
        }


        // Spawn enemies at random locations
        if self.timers.current_time - self.timers.last_spawned_projectile > 2.0 {
            self.timers.last_spawned_projectile = self.timers.current_time;
            
            self.world.generate_new_meteor(&mut self.rng);
            println!("HIT");
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