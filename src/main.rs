use macroquad::prelude::*;

#[macroquad::main("Hello World")]

async fn main() {
    loop{
        if is_key_pressed(KeyCode::W) {
        println!("w pressed")
        }
        next_frame().await
    }
}
