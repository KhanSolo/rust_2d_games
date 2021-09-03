use piston_window::*;

mod draw;
mod snake;
mod game;

fn main() {

    let mut window: PistonWindow =
        WindowSettings::new("Hello World!", [512; 2])
            .build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [10.0, 10.0, 100.0, 100.0], // rectangle
                      c.transform, g);
        });
    }
}