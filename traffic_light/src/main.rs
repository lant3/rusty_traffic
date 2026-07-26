use macroquad::prelude::*;

#[macroquad::main("Traffic Light")]
async fn main() {
    loop {
        clear_background(PURPLE);
        next_frame().await;
    }
}
