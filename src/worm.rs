use std::f32::{consts::PI, EPSILON};
use raylib::prelude::*;

use crate::point::Point;

type Tri<T> = (Point<T>, Point<T>, Point<T>);

struct Bone {
    point: Point<f32>,
    control_radius: f32,
    radius: f32,
    next: Option<Box<Bone>>,
}

pub struct Worm {
    pub color: Color,
    head: Option<Box<Bone>>,
    target: Option<Point<f32>>
}

impl Worm {
    pub fn new() -> Worm {
        Worm { color: Color::MAGENTA, head: None, target: None }
    }

    pub fn grow(&mut self, cr: f32, r: f32) {
        let head = Bone {
            point: Point::zero(),
            control_radius: cr,
            radius: r,
            next: self.head.take(),
        };
        self.head = Some(Box::new(head));
    }

    pub fn find_target_random(&mut self, w: i32, h: i32) {
        // choose a random target at < R
        const R: f32 = 200.;
        let r = rand::random::<f32>()*R;
        let a = rand::random::<f32>()*2.*PI;
        let p = self.head.as_ref().unwrap().point;

        let mut t = p + r*Point::e_i(a);
        // restricted to the window
        let w = w as f32;
        let h = h as f32;
        t.x = if t.x < 0. { 0. } else if t.x > w { w } else { t.x };
        t.y = if t.y < 0. { 0. } else if t.y > h { h } else { t.y };
        self.target = Some(t);
    }

    pub fn roam(&mut self, w: i32, h: i32) -> Vector2 {
        // roams by chaining random targets
        // returns next head position
        const V: f32 = 0.05;
        let p = self.head.as_ref().unwrap().point;

        // update the target if it has been reached
        match self.target {
            None => self.find_target_random(w, h),
            Some(t) =>
                if (t - p).norm() < EPSILON {
                    self.find_target_random(w, h);
                }
        }

        let t = self.target.unwrap();
        let v = t - p;
        if (t - p).norm() < V {
            t.into()
        } else {
            (p + (V/v.norm())*v).into()
        }
    }

    pub fn update(&mut self, Vector2 { x, y }: Vector2) {
        // update the bones with the new head location
        if let Some(head) = &mut self.head {
            head.point = Point { x, y };
            let mut c = head;
            while let Some(next) = &mut c.next {
                next.point = follow(c.point, next.point, c.control_radius);
                c = next;
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let mut tri = Vec::new();
        let mut quad = Vec::new();

        if let Some(head) = &self.head {
            let mut a = head;
            if let Some(b) = &a.next {
                let mut b = b;
                tri_extrema(a, b, &mut tri, &mut quad);

                loop {
                    if let Some(c) = &b.next {
                        tri_bone(a, b, c, &mut tri, &mut quad);
                        a = b;
                        b = c;
                    }
                    else {
                        tri_extrema(b, a, &mut tri, &mut quad);
                        break
                    }
                }
            }
        }
        
        while !tri.is_empty() {
            let (a, b, c) = tri.pop().unwrap();
            d.draw_triangle(a, b, c, self.color);
        }
        
        while !quad.is_empty() {
            let e = quad.pop().unwrap();
            let c = quad.pop().unwrap();
            let b = quad.pop().unwrap();
            let a = quad.pop().unwrap();
            d.draw_triangle(a, b, c, self.color);
            d.draw_triangle(a, c, e, self.color);
        }
    }

    pub fn draw_debug(&self, d: &mut RaylibDrawHandle) {
        let mut c = &self.head;
        while let Some(bone) = c {
            d.draw_circle_lines(bone.point.x as i32, bone.point.y as i32, bone.radius, Color::YELLOW);
            c = &bone.next;
        }
    }
}

fn tri_extrema(ex: &Bone, p: &Bone, tri: &mut Vec<Tri<f32>>, quad: &mut Vec<Point<f32>>) {
    // triangles of head and tail
    const N: u32 = 6;
    let da = PI as f32/(N-1) as f32;
    let n = (ex.point - p.point).normal();
    let n = (-ex.radius/n.norm())*n;

    for i in 0..(N-1) {
        let a = ex.point + n.rotate((i+1) as f32*da);
        let b = ex.point + n.rotate(i as f32*da);
        tri.push((ex.point, a, b));
    }
    
    quad.push(ex.point - n);
    quad.push(ex.point + n);
}

fn tri_bone(a: &Bone, b: &Bone, c: &Bone, tri: &mut Vec<Tri<f32>>, quad: &mut Vec<Point<f32>>) {
    // triangles between bones
    let ab = b.point - a.point;
    let bc = c.point - b.point;
    let n = (ab + bc).normal();
    let n = (b.radius/n.norm())*n;

    let sl = b.point + n;
    let sr = b.point - n;
    // wether turning left or right
    let sgn = if ab.dot(&n) >= 0. { 1. } else { -1. };
    let sa = b.point + sgn*(b.radius/ab.norm())*ab.normal();
    let sc = b.point + sgn*(b.radius/bc.norm())*bc.normal();

    tri.push((sl, sr, sa)); 
    tri.push((sr, sl, sc));
    
    if ab.dot(&n) >= 0. {
        // a
        quad.push(sa);
        quad.push(sr);
        // c
        quad.push(sr);
        quad.push(sc);
    }
    else {
        // a
        quad.push(sl);
        quad.push(sa);
        // c
        quad.push(sc);
        quad.push(sl);
    }
}

fn follow(a: Point<f32>, b: Point<f32>, r: f32) -> Point<f32> {
    // clamp b to a
    let v = b - a;
    let norm = v.norm();
    if norm <= r {
        b
    }
    else {
        a + (r/norm)*v
    }
}