extern crate piston_window;
extern crate piston;
extern crate rand;

use std::cell::RefCell;
use std::env::current_exe;
use piston_window::*;
use std::f64::consts::PI;

// mod wheel;
mod game_grid;
mod game;
mod models;
mod display;

fn main() {
    // Tinker with these for some fun
    let settings = game::GameSettings {
        projectile_spawn_interval: 2.0,
        projectile_initial_speed: 150.0,
        size: 800.0,
        ring_radius: 100.0,
        ring_thickness: 4.0,
        min_projectile_spawn_distance: 350.0,
        max_projectile_spawn_distance: 375.0,
        projectile_radius: 10.0,
        ring_turn_rate: 1.0 * PI
    };

    // Create window
    let window_size = settings.size + 200.0;
    let opengl = OpenGL::V3_2;
    let (width, height) = (window_size as u32, window_size as u32);
    let window: PistonWindow = PistonWindow::build_from_window_settings(
        WindowSettings::new("spin_wheel_game", (width, height + 20))
        .samples(4)
        .vsync(true)
        .exit_on_esc(true)
        .opengl(opengl)

    ).unwrap();

    // Create game 
    let resource_path = current_exe().unwrap().parent().unwrap().to_owned().join("resources/");
    let resources = game::Resources {
        font: RefCell::new(Glyphs::new(
            &resource_path.join("fonts/FiraMono-Bold.ttf"),
            window.factory.borrow().clone()
        ).unwrap())
    };

    let mut game = game::Game::new(settings, resources);


    for e in window {
        match e.event {
            Some(Event::Input(Input::Press(Button::Keyboard(key)))) => {
                game.key_press(key);
            }

            Some(Event::Input(Input::Release(Button::Keyboard(key)))) => {
                game.key_release(key);
            }

            Some(Event::Render(_)) => {
                e.draw_2d(|c, g| {
                	game.render(c.trans(100.0, 100.0), g)
                });
            }

            Some(Event::Update(args)) => {
                game.update(args.dt);
            }

            _ => {}
        }
    }
}