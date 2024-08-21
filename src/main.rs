mod animations;
mod enemies;
mod hero;
mod particles;
mod stars;

use crate::enemies::Enemies;
use crate::hero::Hero;
use crate::stars::Stars;
use crate::particles::Particles;

use macroquad::prelude::*;

fn is_collision(rect1: macroquad::math::Rect, rect2: macroquad::math::Rect) -> bool {
    rect1.x < rect2.x + rect2.w
        && rect1.x + rect1.w > rect2.x
        && rect1.y < rect2.y + rect2.h
        && rect1.y + rect1.h > rect2.y
}

#[macroquad::main("Simple Game")]
async fn main() {
    // Set screen size
    request_new_screen_size(500., 600.);

    let mut stars_spawner = Stars::new();
    let mut enemies_spawner = Enemies::new();
    let mut hero = Hero::new().await;
    let mut particles = Particles::new();

    loop {
        clear_background(BLACK);

        for bullet in hero.bullets.iter_mut() {
        for enemy in enemies_spawner.enemies.iter_mut() {
            let bullet_rect = bullet.2.rects[bullet.2.current_frame as usize].0;
            let enemy_rect = enemy.2.rects[enemy.2.current_frame as usize].0;
            
            let bullet_rect = Rect::new(
                bullet.0, bullet.1,
                bullet_rect.w, bullet_rect.h,
            );

            let enemy_rect = Rect::new(
                enemy.0, enemy.1, 
                enemy_rect.w, enemy_rect.h,
            );
            
            if is_collision(bullet_rect, enemy_rect) && bullet.3 && enemy.3 {
                particles.spawn(bullet_rect.x, bullet_rect.y).await;
                bullet.3 = false;
                enemy.3 = false;
            }
        }}

        hero.bullets.retain(|bullet| bullet.3);
        enemies_spawner.enemies.retain(|enemy| enemy.3);

        // Stars
        stars_spawner.update();
        stars_spawner.draw();

        // Enemies
        enemies_spawner.update().await;
        enemies_spawner.draw();

        // Hero
        hero.update().await;
        hero.draw();

        // Particles
        particles.draw();
        particles.update().await;

        next_frame().await
    }
}
