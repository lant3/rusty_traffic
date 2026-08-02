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
                draw_circle((screen_width() / 2.0) - 100.0, screen_height() / 2.0, 100.0, RED);
            }
            TrafficLightState::Yellow => {
                draw_circle(screen_width() / 2.0, screen_height() / 2.0, 100.0, YELLOW);
            }
            TrafficLightState::Green => {
                draw_circle((screen_width() / 2.0) + 100.0, screen_height() / 2.0, 100.0, GREEN);
            }
        }
    }
    

    fn color_change(&mut self) {
        match self.color {
            TrafficLightState::Red => {
                self.color = TrafficLightState::Yellow;
                self.previous_color = TrafficLightState::Red;
                self.timer = self.timer / 2.0; // Set timer to half of the previous value
            }
            TrafficLightState::Green => {
                self.color = TrafficLightState::Yellow;
                self.previous_color = TrafficLightState::Green;
                self.timer = self.timer / 2.0;
            }
            TrafficLightState::Yellow => {
                match self.previous_color {
                    TrafficLightState::Red => {
                        self.color = TrafficLightState::Green;
                        self.previous_color = TrafficLightState::Yellow;
                        self.timer = self.timer;
                    }
                    TrafficLightState::Green => {
                        self.color = TrafficLightState::Red;
                        self.previous_color = TrafficLightState::Yellow;
                        self.timer = self.timer;
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
    timer: 5.0,};

    let timer: f32 = traffic_light.timer;
    
    loop {
        clear_background(WHITE);
        let delta = get_frame_time();
           
        draw_text(
    &format!("Isabelle & Jack - Time remaining: {:.2}",
            traffic_light.timer),
            screen_width() / 5.0,
            screen_height() / 10.0,
            30.0,
            BLACK
        );
        traffic_light.display_color();

        traffic_light.timer -= delta;
        if traffic_light.timer <= 0.0 {
            traffic_light.timer = timer; // Reset the timer to the original value
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