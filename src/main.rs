mod hero;
mod animations;
mod stars;
mod enemies;

use crate::stars::Stars;
use crate::enemies::Enemies;
use crate::hero::Hero;

use macroquad::prelude::*;

fn is_collision(rect1: macroquad::math::Rect, rect2: macroquad::math::Rect) -> bool {
    rect1.x < rect2.x + rect2.w && rect1.x + rect1.w > rect2.x && rect1.y < rect2.y + rect2.h && rect1.y + rect1.h > rect2.y
}

#[macroquad::main("Simple Game")]
async fn main() {
    // Set screen size
    request_new_screen_size(500., 600.);

    let mut stars_spawner = Stars::new();
    let mut enemies_spawner = Enemies::new();
    let mut hero = Hero::new().await;

    loop {
        clear_background(BLACK);

        for i in (0..hero.bullets.len()).rev() {
            for j in (0..enemies_spawner.enemies.len()).rev() {
                let bullet_rect = hero.bullets[i].2.rects[hero.bullets[i].2.current_frame as usize].0;
                let enemie_rect = enemies_spawner.enemies[j].2.rects[enemies_spawner.enemies[j].2.current_frame as usize].0;
 
                let bullet_rect = Rect::new(hero.bullets[i].0, hero.bullets[i].1, bullet_rect.w, bullet_rect.h);
                let enemie_rect = Rect::new(enemies_spawner.enemies[j].0, enemies_spawner.enemies[j].1, enemie_rect.w, enemie_rect.h);

                if is_collision(bullet_rect, enemie_rect) == true {
                    hero.bullets.remove(i);
                    enemies_spawner.enemies.remove(j);
                }
            }
        }

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
