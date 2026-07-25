use macroquad::prelude::*;

#[macroquad::main("Traffic Light")]
async fn main() {
    loop {
        clear_background(GRAY);
        next_frame().await;
    }
}