use font::{CHARS_HEIGHT, CHARS_SPACE, CHARS_WIDTH, CHAR_MAP};
use rand::prelude::*;

// pub const WIDTH: i32 = 400;
// pub const HEIGHT: i32 = 300;

pub mod font;

pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Rgba {
    pub const WHITE: Self = Self(0xff, 0xff, 0xff, 0xff);
    pub const BLACK: Self = Self(0x0, 0x0, 0x0, 0xff);
    pub const TRANSPARENT: Self = Self(0x0, 0x0, 0x0, 0x0);
    pub const ORANGE: Self = Self(0xff, 0x88, 0x0, 0xff);

    pub const fn as_slice(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
    pub const fn alpha_times(&self, num: u8) -> Self {
        Self(self.0, self.1, self.2, self.3 * num)
    }
    pub const fn all(num: u8) -> Self {
        Self(num, num, num, num)
    }
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0..=255);
        let g = rng.gen_range(0..=255);
        let b = rng.gen_range(0..=255);
        Self(r, g, b, 255)
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Self(0, 0, 0, 0)
    }
}

pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pos<T> {
    #[inline(always)]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[inline]
fn draw_pixel(color: &Rgba, index: usize, frame: &mut [u8]) {
    frame[index..index + 4].copy_from_slice(&color.as_slice());
}

pub fn draw_char(
    char: &[u8; 30],
    color: &Rgba,
    size: usize,
    mut offset: usize,
    frame: &mut [u8],
    frame_width: i32,
) {
    let mut i = 0;
    while i < CHARS_HEIGHT {
        for _ in 0..size {
            let mut j = 0;
            while j < CHARS_WIDTH {
                if char[i * CHARS_WIDTH + j] != 0 {
                    for s in 0..size {
                        let color = color.as_slice();
                        frame[offset + 4 * (s + j * size)..offset + 4 * (s + j * size) + 4]
                            .copy_from_slice(&color);
                    }
                }
                j += 1;
            }
            offset += frame_width as usize * 4;
        }
        i += 1;
    }
}

pub fn draw_text(
    text: &str,
    color: Rgba,
    position: Pos<i32>,
    size: usize,
    frame: &mut [u8],
    frame_width: i32,
) {
    assert_ne!(size, 0);
    let chars = text.chars();
    let mut offset = (4 * (position.y * frame_width + position.x)) as usize;
    for ch in chars {
        if !CHAR_MAP.contains_key(&ch) {
            panic!("{} not supported", ch);
        };
        let char = CHAR_MAP.get(&ch).unwrap();
        draw_char(char, &color, size, offset, frame, frame_width);
        offset += 4 * size * (CHARS_SPACE + CHARS_WIDTH);
    }
}
