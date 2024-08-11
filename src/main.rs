use macroquad::prelude::*;

#[macroquad::main("Simple Game")]
async fn main() {
    loop {
        clear_background(BLACK);

        

        next_frame().await
    }
}
