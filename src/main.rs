use rand::random;
use raylib::prelude::*;

mod worm;
mod point;

use worm::Worm;

fn main() {
    let mut width = 800;
    let mut height = 450;

    let mut w: Worm = Worm::new();
    for i in 0..10 {
        w.grow(10., (5+i/2) as f32);
    }

    let mut worms: Vec<Worm> = Vec::new();
    for _ in 0..30 {
        let mut w = Worm::new();
        let s = 4 + rand::random::<usize>()%20;
        for i in 0..s {
            w.grow(10., (5+i/3) as f32);
        }
        w.color = random_color();
        worms.push(w);
    }

    let (mut rl, thread) = raylib::init()
        .title("")
        .size(width, height)
        .resizable()
        .build();

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            width = rl.get_screen_width();
            height = rl.get_screen_height();
        }

        let p = rl.get_mouse_position();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        for w in worms.iter_mut() {
            let p = w.roam(width, height);
            w.update(p);

            w.draw(&mut d);
        }

        w.update(p);
        w.draw(&mut d);
    }
}

fn random_color() -> Color {
    Color {
        r: random(),
        g: random(),
        b: random(),
        a: 255
    }
}