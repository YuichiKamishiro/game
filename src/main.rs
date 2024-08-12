mod animations;

use crate::animations as anim;
use macroquad::prelude::*;

const STARS_COUNT: i32 = 20;
const STAR_RADIUS: f32 = 3.0;
const WIDTH: f32 = 800.;
const HEIGHT: f32 = 600.;
const STAR_UPDATE_DUR_SEC: f32 = 0.125;

fn update_stars(timer: &mut f32, stars: &mut Vec<(f32, f32)>) {
    if *timer >= STAR_UPDATE_DUR_SEC {
        *timer = 0.;
        for i in 0..STARS_COUNT { 
            let rand_x: f32 = rand::gen_range(0., WIDTH); // using macroquad's rand bc why not???
            let rand_y: f32 = rand::gen_range(0., HEIGHT);
            stars[i as usize] = (rand_x, rand_y);
        }
    }
}

fn draw_stars(stars: &Vec<(f32, f32)>) {
    for i in 0..STARS_COUNT {
        draw_circle(stars[i as usize].0, stars[i as usize].1, STAR_RADIUS, WHITE);
    }
}

#[macroquad::main("Simple Game")]
async fn main() {
    // Set screen size
    request_new_screen_size(WIDTH, HEIGHT);

    let mut timer = 0.;
    let mut fire_timer = 0.;

    let mut stars: Vec<(f32, f32)> = vec![(0., 0.,); STARS_COUNT as usize];
    let mut bullets: Vec<(f32, f32)> = vec![];

    let mut animator = anim::Animator::new(); // Create animator
    animator.load("SpaceshipKit.png").await; // Load spritesheet

    let frames = vec![
        (Rect::new(0., 0., 36., 52.), 0.1),
        (Rect::new(37., 0., 36., 52.), 0.1),
        (Rect::new(72., 0., 36., 52.), 0.1),
    ];

    animator.add_frames(frames);
    // Add frames

    loop {
        clear_background(BLACK);

        // Update clock
        timer = timer + get_frame_time();
        fire_timer = fire_timer + get_frame_time();

        if is_mouse_button_down(MouseButton::Left) && fire_timer >= 0.3 {
            fire_timer = 0.;
            bullets.push((
                mouse_position().0, 
                mouse_position().1 - 60.,
            ));
            // logic
        }

        
        for i in 0..bullets.len() {
            bullets[i].1 = bullets[i].1 - (400. * get_frame_time());
        }
        
        println!("Bullets count: {}", bullets.len());

        // Do it in reverse order to avoid skipping elements
        for i in (0..bullets.len()).rev() {
            if bullets[i].1 <= 0. { bullets.remove(i); }
        }
        // Draw
        draw_stars(&stars);

        // Get current sprite rect so as to center position
        let sprite_rect: Rect = animator.rects[animator.current_frame].0;
        animator.draw(
            mouse_position().0 - sprite_rect.w / 2., 
            mouse_position().1 - sprite_rect.h / 2. - 40.,
        );

        for i in 0..bullets.len() {
            draw_circle(bullets[i].0, bullets[i].1, 3., PINK);
        }

        animator.update();
        // Update stars position
        update_stars(&mut timer, &mut stars);
        next_frame().await
    }
}
