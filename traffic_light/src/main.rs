use std::{thread::sleep, time::Duration};
#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Amber,
    Green,
}

impl TrafficLight {
    fn update(&mut self, ticks: &mut u32) {
        let new_state = match self {
            TrafficLight::Red => {
                if *ticks > 45 {TrafficLight::Amber} else {TrafficLight::Red}
            }
            TrafficLight::Amber => {
                if *ticks > 10 {TrafficLight::Green} else {TrafficLight::Amber}
            }
            TrafficLight::Green => {
                if *ticks > 55 {TrafficLight::Red} else {TrafficLight::Green}
            }
        };
        if new_state != *self {
            *ticks = 0;
            *self = new_state;
        }
    } 
}

fn main() {
    let mut traffic_light = TrafficLight::Red;
    let mut ticks: u32 = 0;
    loop {
        sleep(Duration::from_millis(100));
        ticks += 1;
        traffic_light.update(&mut ticks);
        println!("{:?}, {}", traffic_light, ticks);
    }
}
