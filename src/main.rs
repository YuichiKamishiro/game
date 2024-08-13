mod hero;
mod animations;
mod stars;
mod enemies;

use crate::stars::Stars;
use crate::enemies::Enemies;
use crate::hero::Hero;

use macroquad::prelude::*;

#[macroquad::main("Simple Game")]
async fn main() {
    // Set screen size
    request_new_screen_size(500., 600.);

    let mut stars_spawner = Stars::new();
    let mut enemies_spawner = Enemies::new();
    let mut hero = Hero::new().await;

    loop {
        clear_background(BLACK);

        // Stars
        stars_spawner.update();
        stars_spawner.draw();

        // Enemies
        enemies_spawner.update().await;
        enemies_spawner.draw();

        // Hero
        hero.update().await;
        hero.draw();

        next_frame().await
    }
}
