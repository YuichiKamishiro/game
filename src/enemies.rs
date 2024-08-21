use crate::animations::{AnimationState, Animator};
use macroquad::prelude::*;

static mut SPAWN_PER_TICK: i32 = 2;
static mut TICK_TIME: f32 = 2.;

pub struct Enemies {
    pub enemies: Vec<(f32, f32, Animator, bool)>,
    enemies_timer: f32,
}

impl Enemies {
    pub fn new() -> Self {
        Self {
            enemies: vec![],
            enemies_timer: 0.,
        }
    }

    pub async fn spawn(&mut self) {
        self.enemies_timer = self.enemies_timer + get_frame_time();
        if self.enemies_timer >= unsafe { TICK_TIME } {
            self.update_difficulty();
            self.enemies_timer = 0.;

            let mut texture = Texture2D::empty();
            match load_texture("img/EnemiesSpaceshipKit.png").await {
                Ok(text) => texture = text,
                Err(err) => println!("Error while loading texture: {err}"),
            }

            for _ in 0..unsafe {SPAWN_PER_TICK} {
                let rand_x: f32 = rand::gen_range(20., screen_width() - 20.);
                let mut enemy = Animator::new(AnimationState::Loop);
                enemy.load_from(texture.clone());

                let frames = vec![
                    (Rect::new(0., 0., 36., 52.), 0.1),
                    (Rect::new(37., 0., 36., 52.), 0.1),
                    (Rect::new(74., 0., 36., 52.), 0.1),
                ];

                enemy.add_frames(frames);
                self.enemies.push((rand_x, -50., enemy, true));
            }
        }
    }
    fn change_position(&mut self) {
        for i in 0..self.enemies.len() {
            self.enemies[i].1 = self.enemies[i].1 + (100. * get_frame_time());
        }
    }

    fn update_difficulty(&mut self) {
        unsafe {
            SPAWN_PER_TICK = 2 + (get_time() / 8.) as i32;
        }
    }

    // Update enemies and draw them
    pub async fn update(&mut self) {
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
