use macroquad::prelude::*;
use crate::animations as anim;

pub struct Enemies {
    enemies: Vec<(f32, f32, anim::Animator)>,
    enemies_timer: f32,
}

impl Enemies {
    pub fn new() -> Self {
        Self {
            enemies: vec![],
            enemies_timer: 0.,
        }
    }

    pub async fn spawn_enemies(&mut self) {
        self.enemies_timer = self.enemies_timer + get_frame_time();
        if self.enemies_timer >= 0.5 {
            self.enemies_timer = 0.;
            let rand_x: f32 = rand::gen_range(0., screen_height());
            let mut enemy = anim::Animator::new();
            enemy.load("SpaceshipKit.png").await;

            let frames = vec![
                (Rect::new(0., 0., 36., 52.), 0.1),
                (Rect::new(37., 0., 36., 52.), 0.1),
                (Rect::new(74., 0., 36., 52.), 0.1),
            ];
        
            enemy.add_frames(frames);
            self.enemies.push((rand_x, 20., enemy))
        }
    }
    fn move_enemies(&mut self) {
        for i in 0..self.enemies.len() {
            self.enemies[i].1 = self.enemies[i].1 + (100. * get_frame_time());
        }
    }

    // Update enemies and draw them
    pub fn update_enemies(&mut self) {
        self.move_enemies();

        for i in 0..self.enemies.len() {
            self.enemies[i].2.update();
        }
    }

    pub fn draw_enemies(&self) {
        for i in 0..self.enemies.len() {
            self.enemies[i].2.draw(self.enemies[i].0, self.enemies[i].1);
        }
    }
}
