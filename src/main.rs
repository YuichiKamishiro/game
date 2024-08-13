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
    request_new_screen_size(800., 600.);

    let mut stars_spawner = Stars::new();
    let mut enemies_spawner = Enemies::new();
    let mut hero = Hero::new().await;

    loop {
        clear_background(BLACK);

        // Stars
        stars_spawner.update_stars();
        stars_spawner.draw_stars();

        // Enemies
        enemies_spawner.spawn_enemies().await;
        enemies_spawner.update_enemies();
        enemies_spawner.draw_enemies();

        // Hero
        hero.update();
        hero.draw();

        next_frame().await
    }
}
