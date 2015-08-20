extern crate piston_window;
extern crate graphics;
extern crate piston;
extern crate rand;


use piston_window::*;
use std::f64::consts::PI;

// mod wheel;
mod game_grid;
mod game;
mod models;

fn main() {
    

    let window_size = 800.0;
    // let ring_size = 200.0;
    // let ring_pos = (window_size - ring_size) / 2.0;
    // let turn_rate = 0.005;

    let opengl = OpenGL::V3_2;
    let (width, height) = (window_size as u32, window_size as u32);
    let window: PistonWindow =
        WindowSettings::new("spin_wheel_game", (width, height + 20))
        .samples(4)
        .vsync(true)
        .exit_on_esc(true)
        .opengl(opengl)
        .into();

    let mut game = game::Game::new();

    for e in window {
        match e.event {
            Some(Event::Input(Input::Press(Button::Keyboard(key)))) => {
                game.key_press(key);
            }

            Some(Event::Input(Input::Release(Button::Keyboard(key)))) => {
                game.key_release(key);
            }

            Some(Event::Render(args)) => {
                e.draw_2d(|c, g| {
                	game.render(c, g)
                });
            }

            Some(Event::Update(args)) => {
                game.update(args.dt);
            }

            _ => {}
        }
    }
}