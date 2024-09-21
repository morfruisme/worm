use std::ops::{Add, Mul, Neg, Sub};
use raylib::{math, ffi};

// silly algebra
/*trait Scalar<V: Vector<Self>>: Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<V, Output = V>
{}

trait Vector<S: Scalar<Self>>: Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self> 
{}

impl Scalar<Point<f32>> for f32 {}
impl Vector<f32> for Point<f32> {}*/

#[derive(Clone, Copy)]
pub struct Point<T> { 
    pub x: T, 
    pub y: T 
}

impl Point<f32> {
    pub fn zero() -> Point<f32> {
        Point { x: 0., y: 0. }
    }

    pub fn e_i(alpha: f32) -> Point<f32> {
        Point { x: alpha.cos(), y: alpha.sin() }
    }

    pub fn dot(&self, v: &Point<f32>) -> f32 {
        self.x*v.x + self.y*v.y
    }

    pub fn norm(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normal(&self) -> Self {
        Point { x: -self.y, y: self.x }
    }

    pub fn rotate(&self, a: f32) -> Self {
        Point {
            x: a.cos()*self.x - a.sin()*self.y,
            y: a.sin()*self.x + a.cos()*self.y
        }
    }
}

impl Into<math::Vector2> for Point<f32> {
    fn into(self) -> math::Vector2 {
        math::Vector2 { x: self.x, y: self.y }
    }
}

impl Into<ffi::Vector2> for Point<f32> {
    fn into(self) -> ffi::Vector2 {
        ffi::Vector2 { x: self.x, y: self.y }
    }
}

impl Mul<Point<f32>> for f32 {
    type Output = Point<f32>;

    fn mul(self, rhs: Point<f32>) -> Self::Output {
        Point { x: self*rhs.x, y: self*rhs.y }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T: Neg<Output = T>> Neg for Point<T> {
    type Output = Point<T>;

    fn neg(self) -> Self::Output {
        Point { x: -self.x, y: -self.y }
    }
}