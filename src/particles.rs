use macroquad::prelude::*;
use crate::animations::{AnimationState, Animator};

pub struct Particles {
    pub particles: Vec<(f32, f32, Animator)>,
}

impl Particles {
    pub fn new() -> Self {
        Self {
            particles: vec![],
        }
    }

    pub async fn spawn(&mut self, x: f32, y: f32) {
        let mut particle = Animator::new(AnimationState::Once);
        particle.load("img/Blast.png").await;
        let frames = vec![
            (Rect::new(0., 0., 32., 32.), 0.05),
            (Rect::new(32., 0., 32., 32.), 0.05),
            (Rect::new(64., 0., 32., 32.), 0.05),
            (Rect::new(0., 32., 32., 32.), 0.07),
            (Rect::new(32., 32., 32., 32.), 0.09),
            (Rect::new(64., 32., 32., 32.), 0.12),
            // (Rect::new(0., 64., 32., 32.), 0.2),
            // (Rect::new(32., 64., 32., 32.), 0.1),
            // (Rect::new(64., 64., 32., 32.), 0.05),
        ];

        particle.add_frames(frames);
        self.particles.push((x, y, particle));
    }
    
    pub async fn update(&mut self) {
        for x in 0..self.particles.len() {
            self.particles[x].2.update();
        }
        self.particles.retain(|(_, _, sprite)| {!sprite.is_animatin_stopped});
    }

    pub fn draw(&self) {
        for (x, y, sprite) in &self.particles {
            sprite.draw(*x, *y);
        }
    }
}
