extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use graphics::{DrawState,Transformed,math}; // from piston2d-graphics
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::thread::sleep;

use rand::prelude::*;

pub struct App {
    state: [[bool; 100]; 100],
    next: [[bool; 100]; 100]
}

impl App {
    fn render(&mut self,  _: DrawState, transform: math::Matrix2d,  gfx: &mut GlGraphics) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        const OUTER_CELLSIZE: f64 = 8f64;
        const INNER_CELLSIZE: f64 = 6f64;

        // Clear the screen.
        clear(WHITE, gfx);

        // Draw each cell
        for x in 0..99 {
            for y in (0usize..99usize).filter(|&y| self.state[x][y]) {
                let x = x as f64 * OUTER_CELLSIZE;
                let y = y as f64 * OUTER_CELLSIZE;
                let square = rectangle::square(0f64, 0f64, INNER_CELLSIZE);
                rectangle(BLACK, square, transform.trans(x + 1f64, y + 1f64), gfx);                    
            }
        }
    }

    fn update_cell(&mut self, x: usize, y:usize) {
        // Calculate number of alive neighbours
        const OFFSETS: [(i8,i8); 8] = [(0, -1), (0, 1), (-1, -1), (-1, 0), (-1, 1), (1, -1), (1, 0), (1, 1)];
        let mut neighbours = 0;
        for (offset_x, offset_y) in OFFSETS.iter() {
            let x2 = wrap_around(x as i8 + offset_x);
            let y2 = wrap_around(y as i8 + offset_y);

            if self.state[x2 as usize][y2 as usize] {
                neighbours += 1;
            }            
        }

        // Set cell's alive based on current alive state and number of neighbours
        if self.state[x][y] {
            self.next[x][y] = neighbours == 2 || neighbours == 3;
        } else {
            self.next[x][y] = neighbours == 3;
        }
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.next = [[false; 100]; 100];
        
        for x in 0..100 {
            for y in 0..100 {
                self.update_cell(x, y);
            }
        }
        self.state = self.next;
    }

    fn reset(&mut self) {
        let mut rng = thread_rng();

        for x in 0..99 {
            for y in 0..99 {
                self.state[x][y] = rng.gen_range(0, 3) == 0;
            }
        }
    }
}

fn wrap_around(x: i8) -> i8 {
    if x < 0 {
        x + 100
    } else {
        x % 100
    }
}

fn main() {
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "gameoflife",
            [800, 800]
        )
        .opengl(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        
        state: [[false; 100]; 100],
        next: [[false; 100]; 100]
    };

    let mut gfx = GlGraphics::new(OpenGL::V3_2);

    // // Spinner
    // app.state[10][10] = true;
    // app.state[10][11] = true;
    // app.state[10][12] = true;

    // // Glider
    // app.state[94][93] = true;
    // app.state[95][94] = true;
    // app.state[93][95] = true;
    // app.state[94][95] = true;
    // app.state[95][95] = true;

    app.reset();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gfx.draw(r.viewport(), |context, gfx| {
                // Scale the drawing context with the window
                let size = context.get_view_size();                
                let context = context.scale(size[0] / 800f64, size[1] / 800f64);

                sleep(std::time::Duration::from_millis(50));
                app.render(context.draw_state, context.transform, gfx);
            });
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::R {
                println!("Reset!");
                app.reset();
            }

            println!("Pressed keyboard key '{:?}'", key);
        };
    }
}
