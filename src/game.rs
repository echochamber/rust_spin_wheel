use std::cell::RefCell;
use rand::{self, ThreadRng, Rng};
use std::f64::consts::PI;
use piston_window::*;
use models::*;
use game_grid::*;
use display::*;


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
    settings: GameSettings,

    /// Resources, font ect...
    resources: Resources
}


/// Active actions (toggled by user input)
#[derive(Default)]
struct Actions {
    rotate_clockwise: bool,
    rotate_c_clockwise: bool,
    move_up: bool,
    move_left: bool,
    move_right: bool,
    move_down: bool
}

/// Timers to handle creation of bullets, enemies and particles
#[derive(Default)]
struct Timers {
    current_time: f64,
    last_spawned_projectile: f64
}

/// Additional resources needed for the game
pub struct Resources {
    pub font: RefCell<Glyphs>
}

#[derive(Clone)]
pub struct GameSettings {
    /// Interval between new projectile spawns, in seconds
    pub projectile_spawn_interval: f64,

    /// Projectile initial speed, units/second
    pub projectile_initial_speed: f64,

    /// Length of the square containing the game, in units
    pub size: f64,

    /// Inclusive
    pub max_projectile_spawn_distance: f64,

    /// Inclusive
    pub min_projectile_spawn_distance: f64,

    pub projectile_radius: f64,

    /// thickness of the ring
    pub ring_thickness: f64,

    /// Units
    pub ring_radius: f64,

    /// Radians per second
    pub ring_turn_rate: f64,

    pub ring_move_speed: f64
}

impl Game {
	pub fn new(settings: GameSettings, resources: Resources) -> Game {
        let rng = rand::thread_rng();
        Game {
            world: World::new(
                settings.size,
                Ring::new((settings.size / 2.0, settings.size / 2.0).into(), settings.ring_radius, settings.ring_thickness, settings.ring_turn_rate)
            ),
            score: 0,
            actions: Actions::default(),
            timers: Timers::default(),
            rng: rng,
            settings: settings,
            resources: resources
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {

        clear([0.55, 0.55, 0.55, 1.0], g);

        // Render the score
        {
            let text = Text::new_color(GameColors::Orange.into(), 22);
            text.draw(&format!("Score: {}", self.score),
                &mut *self.resources.font.borrow_mut(),
                &c.draw_state,
                c.trans(10.0, 20.0).transform,
                g
            );
        }

        let border = Rectangle::new_border([0.0, 0.0, 0.0, 1.0], 2.0);
        border.draw(
            [0.0, 0.0, self.settings.size, self.settings.size],
            &c.draw_state,
            c.transform,
            g
        );

        self.world.render(c, g);
    }

    pub fn center(&self) -> (f64, f64) {
    	(self.settings.size / 2.0, self.settings.size / 2.0)
    }

    pub fn update(&mut self, dt: f64) {
        self.timers.current_time += dt;

        // Handle rotation of the inner circle
        if self.actions.rotate_c_clockwise {
            self.world.ring.turn_time(-dt);
        }
        if self.actions.rotate_clockwise {
            self.world.ring.turn_time(dt);
        }

        self.update_projectiles(dt);

        self.new_projectiles();
    }

    fn update_projectiles(&mut self, dt: f64) {
        for projectile in &mut self.world.projectiles {
            projectile.move_time(dt);
        }

        {
            let center = self.center().into();
            let ring_radius = self.settings.ring_radius;
            let ring_thickness = self.settings.ring_thickness;
            let projectile_radius = self.settings.projectile_radius;
            let ring = &self.world.ring;
            let mut scored = 0;
            self.world.projectiles.retain(|projectile|
                {
                    let retain = projectile.position.squared_distance_to(&center).sqrt() > ring_radius + (projectile_radius + ring_thickness) / 2.0;

                    if !retain  {
                        if ring.get_color_at_radian(projectile.position.angle_between(&ring.position)) == projectile.color {
                            scored += 50;
                        };
                    }
                    
                    retain
                }
            );

            self.score += scored;
        }
    }

    fn new_projectiles(&mut self) {
        if self.timers.current_time - self.timers.last_spawned_projectile > self.settings.projectile_spawn_interval {

            let theta = self.rng.gen_range(0.0, 2.0) * PI;
            let r = self.rng.gen_range(self.settings.min_projectile_spawn_distance, self.settings.max_projectile_spawn_distance);
            let p: Point = Point::new_offset_polar(&self.world.center(), r, theta);
            let direction = self.world.center().angle_between(&p);
            let projectile = Projectile::new(
                p.clone(),
                direction,
                self.settings.projectile_initial_speed,
                self.settings.projectile_radius,
                self.rand_color()
            );

            self.world.projectiles.push(projectile);
            self.timers.last_spawned_projectile = self.timers.current_time;
        }
    }

    fn rand_color(&mut self) -> GameColors {
        match self.rng.gen_range(0, 4) {
            0 => { GameColors::Red },
            1 => { GameColors::Green },
            2 => { GameColors::Blue },
            3 => { GameColors::Yellow },
            _ => { panic!("Rng generated a number outside expected range? Wtf?") }
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