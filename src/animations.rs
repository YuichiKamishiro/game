use macroquad::prelude::*;

#[derive(Clone)]
pub struct SpriteSheet {
    pub texture: Texture2D,
}

impl SpriteSheet {
    pub fn new() -> Self {
        SpriteSheet {
            texture: Texture2D::empty(),
        }
    }

    pub async fn load(&mut self, path: &str) {
        match load_texture(path).await {
            Ok(texture) => self.texture = texture,
            Err(err) => println!("Error while loading texture: {err}"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum AnimationState{
    Loop,
    Once,
}

#[derive(Clone)]
pub struct Animator {
    pub sprite_sheet: SpriteSheet,
    pub current_time: f32,
    pub current_frame: usize,
    pub rects: Vec<(Rect, f32)>,
    state: AnimationState,
    pub is_animatin_stopped: bool,
}

impl Animator {
    pub fn new(state: AnimationState) -> Self {
        Animator {
            sprite_sheet: SpriteSheet::new(),
            current_time: 0.,
            current_frame: 0,
            rects: Vec::new(),
            state,
            is_animatin_stopped: false
        }
    }
    pub async fn load(&mut self, path: &str) {
        self.sprite_sheet.load(path).await
    }
    pub fn add_frames(&mut self, rects: Vec<(Rect, f32)>) {
        self.rects = rects;
    }
    pub fn update(&mut self) {
        if !self.is_animatin_stopped {
            self.current_time = self.current_time + get_frame_time();

            if self.current_time >= self.rects[self.current_frame].1 {
                self.current_time = 0.;
                self.current_frame = self.current_frame + 1;
            }
            if self.current_frame == self.rects.len() {
                if self.state == AnimationState::Once { self.is_animatin_stopped = true; }
                self.current_frame = 0;
            }
        }
    }
    pub fn draw(&self, x: f32, y: f32) {
        if self.rects.is_empty() {
            println!("Error frames weren't added, make sure you called add_frames() func");
            return;
        }
        draw_texture_ex(&self.sprite_sheet.texture, x, y, WHITE, DrawTextureParams {
            source: Some(self.rects[self.current_frame].0),
            ..Default::default()
        });
    }
}