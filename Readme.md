# Color Catcher Game

Simple game made with [piston](https://github.com/PistonDevelopers/piston). Particals spawn and fall towards the ring. Try to catch the particals on their matching color.

Controls:

* Up: Rotate the ring counter clockwise
* Down: Rotate the ring clockwise
* Esc: Exit the game

## Running

The only dependency not included in the crate is SDL2. [The rust-sdl2 crate README has an easy explanation for installing sdl2 on Linux, OSX, or Windows](https://github.com/AngryLawyer/rust-sdl2). If you don't want to install SDL2, you can use any of the other implementations of piston_window. [Check out the piston_window README](https://github.com/PistonDevelopers/piston_window) for examples on how to easily swap out implementations.

Aside from that, just `cargo run` and it should be good to go. Feel free to change the settings in main.rs and see how the gameplay changes.


## Thank yous

[aochagavia](https://github.com/aochagavia) for writing [rocket](https://github.com/aochagavia/rocket) which was extremely helpful to reference.
