use macroquad::prelude::*;

const STARS_COUNT: i32 = 60;
const STAR_RADIUS: f32 = 3.0;
const STAR_UPDATE_DUR_SEC: f32 = 0.250;

pub struct Stars {
    stars: Vec<(f32, f32)>,
    timer: f32,
}

impl Stars {
    pub fn new() -> Self{
        Self{ stars: vec![(0., 0.,); STARS_COUNT as usize], timer: 0.}
    }

    // Update stars position
    pub fn update(&mut self) {
        self.timer = self.timer + get_frame_time();
        if self.timer >= STAR_UPDATE_DUR_SEC {
            self.timer = 0.;
            for i in 0..STARS_COUNT { 
                let rand_x: f32 = rand::gen_range(0., screen_width()); // using macroquad's rand bc why not???
                let rand_y: f32 = rand::gen_range(0., screen_height());
                self.stars[i as usize] = (rand_x, rand_y);
            }
        }
    }

    // Draw stars
    pub fn draw(&self) {
        for i in 0..STARS_COUNT {
            draw_circle(self.stars[i as usize].0, self.stars[i as usize].1, STAR_RADIUS, WHITE);
        }
    }
}