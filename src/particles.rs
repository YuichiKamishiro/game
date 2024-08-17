use macroquad::prelude::*;
use crate::animations as anim;

pub struct Particles {
    pub particles: Vec<(f32, f32, anim::Animator)>,
}

impl Particles {
    pub fn new() -> Self {
        Self {
            particles: vec![],
        }
    }

    pub async fn spawn(&mut self, x: f32, y: f32) {
        let mut particle = anim::Animator::new(anim::AnimationState::Once);
        particle.load("Boom.png").await;
        let frames = vec![
            (Rect::new(0., 0., 32., 32.), 0.2),
            (Rect::new(0., 0., 32., 32.), 0.2),
            (Rect::new(0., 0., 32., 32.), 0.2),
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
