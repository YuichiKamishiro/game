mod animations;

use crate::animations as anim;
use macroquad::prelude::*;

const STARS_COUNT: i32 = 35;
const STAR_RADIUS: f32 = 3.0;
const WIDTH: f32 = 800.;
const HEIGHT: f32 = 600.;
const STAR_UPDATE_DUR_SEC: f32 = 0.07;

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
    let mut timer = 0.;
    let mut stars: Vec<(f32, f32)> = vec![(0., 0.,); STARS_COUNT as usize];

    let mut animator = anim::Animator::new(); // Create animator
    animator.load("sprite.png").await; // Load spritesheet

    let frames = vec![
        (Rect::new(0., 0., 120., 80.), 0.5),
        (Rect::new(120., 0., 120., 80.), 0.1),
        (Rect::new(240., 0., 120., 80.), 0.1),
        (Rect::new(360., 0., 120., 80.), 0.3),
    ];

    animator.add_frames(frames);
    // Add frames

    loop {
        clear_background(BLACK);

        // Update clock
        timer = timer + get_frame_time();

        // Update stars position
        update_stars(&mut timer, &mut stars);

        // Draw
        draw_stars(&stars);

        next_frame().await
    }
}
