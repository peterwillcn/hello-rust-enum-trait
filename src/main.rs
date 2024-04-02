trait TrafficLightTimer {
    fn time(&self) -> u8;
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLightTimer for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 35,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 45,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    println!("Red light time: {} s", red_light.time());
    println!("Yellow light time: {} s", yellow_light.time());
    println!("Green light time: {} s", green_light.time());
}
