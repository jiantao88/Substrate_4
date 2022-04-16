fn main() {
    let trafficLight = TrafficLight::Green;
    println!("交通灯: {}",trafficLight.calculateTime());
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

pub trait CallBackTime{
    fn calculateTime(&self)->String;
}

impl CallBackTime for TrafficLight {
    fn calculateTime(&self)->String {
        match self {
            TrafficLight::Red =>String::from("红灯亮10s"),
            TrafficLight::Green =>String::from("绿灯亮15s"),
            TrafficLight::Yellow =>String::from("黄灯亮13s"),
            _=>String::from("")
        }
    }
}