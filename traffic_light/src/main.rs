use macroquad::{input::KeyCode::T, prelude::*};

// Traffic light state
enum TrafficLightState {
    Red,
    Yellow,
    Green,
}

struct TrafficLight {
        color: TrafficLightState,
        previous_color: TrafficLightState,
        timer: f32,
    }

impl TrafficLight {
    fn display_color(&self) {
        match self.color {
            TrafficLightState::Red => {
                draw_circle(screen_width() / 2.0, screen_height() / 2.0, 50.0, RED);
            }
            TrafficLightState::Yellow => {
                draw_circle(screen_width() / 2.0, screen_height() / 2.0, 50.0, YELLOW);
            }
            TrafficLightState::Green => {
                draw_circle(screen_width() / 2.0, screen_height() / 2.0, 50.0, GREEN);
            }
        }
    }

    fn color_change(&mut self) {
        match self.color {
            TrafficLightState::Red => {
                self.color = TrafficLightState::Yellow;
                self.previous_color = TrafficLightState::Red;
                self.timer = 2.0;
            }
            TrafficLightState::Green => {
                self.color = TrafficLightState::Yellow;
                self.previous_color = TrafficLightState::Green;
                self.timer = 2.0;
            }
            TrafficLightState::Yellow => {
                match self.previous_color {
                    TrafficLightState::Red => {
                        self.color = TrafficLightState::Green;
                        self.previous_color = TrafficLightState::Yellow;
                        self.timer = 2.0;
                    }
                    TrafficLightState::Green => {
                        self.color = TrafficLightState::Red;
                        self.previous_color = TrafficLightState::Yellow;
                        self.timer = 2.0;
                    }
                    TrafficLightState::Yellow => {
                        // This case should not happen, but we can handle it gracefully
                        self.color = TrafficLightState::Red;
                        self.previous_color = TrafficLightState::Yellow;
                        self.timer = 2.0;
                    }
                    
            }
        }
        }
    }
}


#[macroquad::main("Traffic Light")]
async fn main() {
    
    let mut traffic_light = TrafficLight {
    color: TrafficLightState::Red,
    previous_color: TrafficLightState::Yellow,
    timer: 4.0,};
    
    loop {
        clear_background(GRAY);
        let delta = get_frame_time();
        traffic_light.display_color();
        traffic_light.timer -= delta;
        if traffic_light.timer <= 0.0 {
            traffic_light.color_change();
        }
        next_frame().await;
    }

    

}






// let traffic_light_red = TrafficLight {
//     color: TrafficLightState::Red,
//     timer: 4.0,
// };

// let traffic_light_amber = TrafficLight {
//     color: TrafficLightState::Amber,
//     timer: 4.0,
// };

// let traffic_light_green = TrafficLight {
//     color: TrafficLightState::Green,
//     timer: 4.0,
// };