mod animations;
mod stars;
mod enemies;

use crate::animations as anim;
use crate::stars::Stars;
use crate::enemies::Enemies;

use macroquad::prelude::*;

#[macroquad::main("Simple Game")]
async fn main() {
    // Set screen size
    request_new_screen_size(800., 600.);

    let mut stars_spawner = Stars::new();
    let mut enemies_spawner = Enemies::new();

    let mut fire_timer = 0.;

    let mut bullets: Vec<(f32, f32)> = vec![];

    let mut animator = anim::Animator::new(); // Create animator
    animator.load("SpaceshipKit.png").await; // Load spritesheet

    let frames = vec![
        (Rect::new(0., 0., 36., 52.), 0.1),
        (Rect::new(37., 0., 36., 52.), 0.1),
        (Rect::new(74., 0., 36., 52.), 0.1),
    ];

    animator.add_frames(frames);
    // Add frames

    loop {
        clear_background(BLACK);

        // Update clock
        fire_timer = fire_timer + get_frame_time();

        if is_mouse_button_down(MouseButton::Left) && fire_timer >= 0.3 {
            fire_timer = 0.;
            bullets.push((
                mouse_position().0, 
                mouse_position().1 - 60.,
            ));
        }
        
        for i in 0..bullets.len() {
            bullets[i].1 = bullets[i].1 - (400. * get_frame_time());
        }

        // Do it in reverse order to avoid skipping elements
        for i in (0..bullets.len()).rev() {
            if bullets[i].1 <= 0. { bullets.remove(i); }
        }

        // Draw bullets
        for i in 0..bullets.len() {
            draw_circle(bullets[i].0, bullets[i].1, 3., PINK);
        }

        // Stars
        stars_spawner.update_stars();
        stars_spawner.draw_stars();

        // Enemies
        enemies_spawner.spawn_enemies().await;
        enemies_spawner.update_enemies();
        enemies_spawner.draw_enemies();

        // Hero
        animator.update();
        // Get current sprite rect so as to center position
        let sprite_rect: Rect = animator.rects[animator.current_frame].0;
        animator.draw(
            mouse_position().0 - sprite_rect.w / 2., 
            mouse_position().1 - sprite_rect.h / 2. - 40.,
        );

        next_frame().await
    }
}
