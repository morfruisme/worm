use raylib::prelude::*;

mod worm;
mod point;

use worm::Worm;

fn main() {
    let mut w: Worm = Worm::new();
    for i in 0..10 {
        w.grow(10., (5+i/2) as f32);
    }

    let mut worms: Vec<Worm> = Vec::new();
    for _ in 0..30 {
        let mut w = Worm::new();
        let s = 4 + rand::random::<usize>()%10;
        for i in 0..s {
            w.grow(10., (5+i/2) as f32);
        }
        worms.push(w);
    }

    let (mut rl, thread) = raylib::init()
        .title("")
        .size(800, 450)
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        for w in worms.iter_mut() {
            //w.update(rl.get_mouse_position());
            let p = w.roam();
            w.update(p);

            //w.draw(&mut d);
            w.draw_outline(&mut d);
        }
    }
}
