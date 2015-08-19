extern crate piston_window;
extern crate graphics;
extern crate piston;

use piston_window::*;

mod wheel;

fn main() {

    // Variables for the piston window
    let opengl = OpenGL::V3_2;
    let (width, height) = (800, 800);
    let window: PistonWindow =
        WindowSettings::new("spin_wheel_game", (width, height))
        //.samples(4)
        //.vsync(true)
        .exit_on_esc(true)
        .opengl(opengl)
        .into();

    let mut current = 60.0;
    let mut pressed = true;
    for e in window {
        match e.event {
            Some(Event::Render(_)) => {
                e.draw_2d(|c, g| {
                    clear([1.0, 1.0, 1.0, 1.0], g);
                    arc(
                    	[0.3, 0.3, 0.6, 1.0],
                    	4.0,
                    	::std::f64::consts::PI * 0.25,
                    	::std::f64::consts::PI * 1.25,
                    	[370.0, 370.0, 60.0, 60.0],
						c.transform,
						g
					);

     //                let elipse = Ellipse::new_border([0.3, 0.3, 0.6, 1.0], 4.0)
     //                	.resolution(512);

     //                let pos = (width as f64 - current) / 2.0;

     //                Ellipse::draw(
					// 	&elipse,
					// 	[pos, pos, current, current],
					// 	&c.draw_state,
					// 	c.transform,
					// 	g
					// );
                });
            },
            Some(Event::Update(_)) => {
            	if pressed {
            		current = current + (current * 0.005);
            	}
            	if current > 800.0 {
            		current = 60.0;
            	}
            },
            Some(Event::Input(Input::Press(Button::Keyboard(Key::Space)))) => {
                pressed = !pressed;
            },
            
            _ => {}
        }
    }
}