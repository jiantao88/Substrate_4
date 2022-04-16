use std::fmt::Display;
use std::ops::{Add, Mul};
use std::{f64::consts::PI, convert::Into};

fn main() {
    let trafficLight = TrafficLight::Green;
    //Q1
    println!("交通灯: {}", trafficLight.calculateTime());
    //Q2
    println!("sum is {:?}", sum_u32(&[100, 1, 12, u32::MAX]));
    //Q3
    let round1 = Round { r: 3 };
    round1.area();
    let triangle = Triangle {a: 4,b:3,c:5};
    triangle.area();
    let square = Square{width:4};
    square.area();
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

pub trait CallBackTime {
    fn calculateTime(&self) -> String;
}

impl CallBackTime for TrafficLight {
    fn calculateTime(&self) -> String {
        match self {
            TrafficLight::Red => String::from("红灯亮10s"),
            TrafficLight::Green => String::from("绿灯亮15s"),
            TrafficLight::Yellow => String::from("黄灯亮13s"),
            _ => String::from(""),
        }
    }
}

fn sum_u32(input: &[u32]) -> Option<u32> {
    let mut total: u32 = 0;
    for i in input.iter() {
        if total.checked_add(*i) == None {
            return None;
        }
        total = total + i;
    }
    Some(total)
}

struct Round<T> {
    r: T,
}
struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}
struct Square<T> {
    width: T,
}

pub trait Area {
    fn area(&self);
}

impl<T: Into<f64> + Copy + Mul<Output = T> + Display> Area for Round<T> {
    fn area(&self) {
        let rr = self.r * self.r;
        let area = T::into(rr) * PI;
        println!("圆形面积={}", area);
    }
}

impl<T: Into<f64> + Add<Output = T> + Copy> Area for Triangle<T> {
    fn area(&self) {
        let abc = self.a + self.b + self.c;
        let p = T::into(abc) / 2.0;
        let area = (p * (p - T::into(self.a)) * (p - T::into(self.b)) * (p - T::into(self.c))).sqrt();
        println!("三角形面积={}", area); 
    }
}

impl<T: Into<f64> + Mul<Output = T> + Copy + Display> Area for Square<T> {
    fn area(&self)  {
        let area = self.width * self.width;
        println!("正方形面积={}", area); 
    }
}
