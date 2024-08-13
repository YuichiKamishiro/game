use macroquad::prelude::*;
use crate::animations as anim;

pub struct Enemies {
    pub enemies: Vec<(f32, f32, anim::Animator)>,
    enemies_timer: f32,
    enemies_spawn_time_per_sec: f32,
}

impl Enemies {
    pub fn new() -> Self {
        Self {
            enemies: vec![],
            enemies_timer: 0.,
            enemies_spawn_time_per_sec: 2., 
        }
    }

    pub async fn spawn(&mut self) {
        self.enemies_timer = self.enemies_timer + get_frame_time();
        if self.enemies_timer >=  self.enemies_spawn_time_per_sec{
            self.enemies_timer = 0.;
            let rand_x: f32 = rand::gen_range(20., screen_width() - 20.);
            let mut enemy = anim::Animator::new();
            enemy.load("EnemiesSpaceshipKit.png").await;

            let frames = vec![
                (Rect::new(0., 0., 36., 52.), 0.1),
                (Rect::new(37., 0., 36., 52.), 0.1),
                (Rect::new(74., 0., 36., 52.), 0.1),
            ];
        
            enemy.add_frames(frames);
            self.enemies.push((rand_x, -50., enemy))
        }
    }
    fn change_position(&mut self) {
        for i in 0..self.enemies.len() {
            self.enemies[i].1 = self.enemies[i].1 + (100. * get_frame_time());
        }
    }

    fn update_spawn_time(&mut self) {
        self.enemies_spawn_time_per_sec = 2.0 - (get_time() as f32 / 20.);
    }

    // Update enemies and draw them
    pub async fn update(&mut self) {
        self.update_spawn_time();
        self.spawn().await;
        self.change_position();

        for i in 0..self.enemies.len() {
            self.enemies[i].2.update();
        }
    }

    pub fn draw(&self) {
        for i in 0..self.enemies.len() {
            self.enemies[i].2.draw(self.enemies[i].0, self.enemies[i].1);
        }
    }
}
