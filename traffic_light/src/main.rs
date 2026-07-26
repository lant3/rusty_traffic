use macroquad::prelude::*;

#[macroquad::main("Traffic Light")]
async fn main() {
    loop {
        clear_background(GREEN);
        next_frame().await;
    }
}