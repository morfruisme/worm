use std::f32::{consts::PI, EPSILON};
use raylib::prelude::*;

use crate::point::Point;

struct Bone {
    point: Point<f32>,
    control_radius: f32,
    radius: f32,
    next: Option<Box<Bone>>,
}

pub struct Worm {
    color: Color,
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

    pub fn find_target_random(&mut self) {
        const R: f32 = 200.;
        let r = rand::random::<f32>()*R;
        let a = rand::random::<f32>()*2.*PI;
        let h = self.head.as_ref().unwrap().point;

        let mut t = h + r*Point::e_i(a);
        t.x = if t.x < 0. { 0. } else if t.x > 800. { 800. } else { t.x };
        t.y = if t.y < 0. { 0. } else if t.y > 450. { 450. } else { t.y };
        self.target = Some(t);
    }

    pub fn roam(&mut self) -> Vector2 {
        const V: f32 = 0.01;
        let h = self.head.as_ref().unwrap().point;

        match self.target {
            None => self.find_target_random(),
            Some(t) =>
                if (t - h).norm() < EPSILON {
                    self.find_target_random();
                }
        }

        let t = self.target.unwrap();
        let v = t - h;
        if (t - h).norm() < V {
            t.into()
        } else {
            (h + (V/v.norm())*v).into()
        }
    }

    pub fn update(&mut self, Vector2 { x, y }: Vector2) {
        if let Some(head) = &mut self.head {
            head.point = Point { x, y };
            let mut c = head;
            while let Some(next) = &mut c.next {
                next.point = follow(c.point, next.point, c.control_radius);
                c = next;
            }
        }
    }

    pub fn draw_outline(&self, d: &mut RaylibDrawHandle) {
        let mut left: Vec<Point<f32>> = Vec::new();
        let mut right: Vec<Point<f32>> = Vec::new();

        if let Some(head) = &self.head {
            let mut a = head;
            if let Some(b) = &a.next {
                let mut b = b;
                extrema(a, b, &mut right);

                loop {
                    if let Some(c) = &b.next {
                        joint(a, b, c, &mut left, &mut right);
                        a = b;
                        b = c;
                    }
                    else {
                        extrema(b, a, &mut right);
                        break
                    }
                }
            }
        }

        left.reverse();
        let side = [left, right].concat();

        for i in 1..side.len() {
            d.draw_line_v(side[i-1], side[i], self.color);
        }
        d.draw_line_v(side[side.len()-1], side[0], self.color);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let mut c = &self.head;
        while let Some(bone) = c {
            d.draw_circle_lines(bone.point.x as i32, bone.point.y as i32, bone.radius, Color::YELLOW);
            c = &bone.next;
        }
    }
}

fn extrema(ex: &Bone, p: &Bone, right: &mut Vec<Point<f32>>) {
    const N: i32 = 6;
    let da = PI as f32/(N-1) as f32;
    let n = (ex.point - p.point).normal();
    let n = (-ex.radius/n.norm())*n;
    
    for i in 0..N {
        right.push(ex.point + n.rotate(i as f32*da));
    }
}

fn joint(a: &Bone, b: &Bone, c: &Bone, left: &mut Vec<Point<f32>>, right: &mut Vec<Point<f32>>) {
    let ab = b.point - a.point;
    let bc = c.point - b.point;
    let n = (ab + bc).normal();
    let n = (b.radius/n.norm())*n;

    if ab.dot(&n) >= 0. {
        left.push(b.point + (b.radius/ab.norm())*ab.normal());
        left.push(b.point + n);
        left.push(b.point + (b.radius/bc.norm())*bc.normal());
        right.push(b.point - n);
    }
    else {
        left.push(b.point + n);
        right.push(b.point - (b.radius/ab.norm())*ab.normal());
        right.push(b.point - n);
        right.push(b.point - (b.radius/bc.norm())*bc.normal());
    }
}

fn follow(a: Point<f32>, b: Point<f32>, r: f32) -> Point<f32> {
    let v = b - a;
    let norm = v.norm();
    if norm <= r {
        b
    }
    else {
        a + (r/norm)*v
    }
}