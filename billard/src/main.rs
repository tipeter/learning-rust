// Crate-level setting. Without '!' it is only local.
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::ops::{Add, Div, Mul, Sub};
use std::fmt::Display;

struct Desktop<T> {
    width: T,
    height: T,
}

struct Ball<'a, T>
where
    T: Display + Clone + Add + Sub + Mul + Div + Into<f64>,
{
    x: T,
    y: T,
    ax: T,
    ay: T,
    moving: Box<Fn(&Ball<T>) -> (T, T) + 'a>,
    fv: fn(&Ball<T>) -> (T, T),
    desktop: &'a Desktop<T>
}

impl<'a, T> Ball<'a, T>
where
    T: Display + Clone + Add + Sub + Mul + Div + Into<f64>,
{
    pub fn new(p_x: T, p_y: T, p_ax: T, p_ay: T, p_desk: &'a Desktop<T>) -> Self {
        Ball {
            x: p_x,
            y: p_y,
            ax: p_ax,
            ay: p_ay,
            moving: Box::new(|b| (b.x.clone(), b.y.clone())),
            fv: |b| {(b.x.clone(), b.y.clone())},
            desktop: p_desk
        }
    }

    pub fn set_moving_func<F: Fn(&Ball<T>) -> (T, T) + 'a>(&mut self, p_moving: F) {
        self.moving = Box::new(p_moving);
    }

    pub fn do_move(&mut self) -> (T, T) {
        let (x, y) = self.moving.as_ref()(&self);
        self.x = x.clone();
        self.y = y.clone();
        (x, y)
    }

    pub fn set_fv(&mut self, p_moving: fn(&Ball<T>) -> (T, T)) {
        self.fv = p_moving;
    }

    pub fn do_fv(&mut self) -> (T, T) {
        let (x, y) = (self.fv)(&self);
        self.x = x.clone();
        self.y = y.clone();
        (x, y)
    }
}

fn main() {
    let desk = Desktop {width: 100.0, height: 100.0};
    let mut ball = Ball::new(0.0, 0.0, 1.2, 1.2, &desk);

    println!("x,y = {:?}", ball.do_move());

    let msg1 = String::from("Message 1");
    let msg2 = String::from("Message 2");

    ball.set_moving_func(move |b|
        {
            println!("prev. (x, y) = ({}, {})", b.x, b.y);
            println!("{:?}", msg1);
            (b.x + b.ax, b.y + b.ay)
        });

    println!("x,y = {:?}", ball.do_move());
    println!("x,y = {:?}", ball.do_move());

    println!("{:?}", msg2);

    ball.set_fv(move |b|
        {
            println!("prev. (x, y) = ({}, {})", b.x, b.y);
//            println!("{:?}", msg2);
            (b.x + b.ax, b.y + b.ay)
        });

    println!("x,y = {:?}", ball.do_fv());
    println!("x,y = {:?}", ball.do_fv());
}
