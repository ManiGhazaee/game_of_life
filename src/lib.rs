#[derive(Clone)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Rgba {
    pub const WHITE: Self = Self(0xff, 0xff, 0xff, 0xff);
    pub const BLACK: Self = Self(0x0, 0x0, 0x0, 0xff);
    pub const TRANSPARENT: Self = Self(0x0, 0x0, 0x0, 0x0);
    pub const ORANGE: Self = Self(0xff, 0x88, 0x0, 0xff);
    pub const RED: Self = Self(0xff, 0x0, 0x0, 0xff);
    pub const BLUE: Self = Self(0x00, 0x33, 0xff, 0xff);
    pub const CYAN: Self = Self(0x22, 0xdd, 0xff, 0xff);
    pub const GREEN: Self = Self(0x00, 0xff, 0x00, 0xff);
    pub const YELLOW: Self = Self(0xff, 0xff, 0x00, 0xff);
    pub const PURPLE: Self = Self(0x80, 0x00, 0x80, 0xff);
    pub const PINK: Self = Self(0xff, 0x69, 0xb4, 0xff);
    pub const TEAL: Self = Self(0x00, 0x80, 0x80, 0xff);

    pub const fn as_slice(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
    pub const fn with_alpha(&self, num: u8) -> Self {
        Self(self.0, self.1, self.2, num)
    }
}

pub struct MixedRgba(pub Rgba, pub Rgba);

impl MixedRgba {
    pub fn as_rgba(&self, mix: u8) -> Rgba {
        let mix_ratio = mix as f32 / u8::MAX as f32;

        let diff0 = (self.1 .0 as f32 - self.0 .0 as f32).abs();
        let diff1 = (self.1 .1 as f32 - self.0 .1 as f32).abs();
        let diff2 = (self.1 .2 as f32 - self.0 .2 as f32).abs();
        let diff3 = (self.1 .3 as f32 - self.0 .3 as f32).abs();

        let ret = Rgba(
            if self.0 .0 > self.1 .0 {
                self.0 .0 - (diff0 * mix_ratio) as u8
            } else {
                self.0 .0 + (diff0 * mix_ratio) as u8
            },
            if self.0 .1 > self.1 .1 {
                self.0 .1 - (diff1 * mix_ratio) as u8
            } else {
                self.0 .1 + (diff1 * mix_ratio) as u8
            },
            if self.0 .2 > self.1 .2 {
                self.0 .2 - (diff2 * mix_ratio) as u8
            } else {
                self.0 .2 + (diff2 * mix_ratio) as u8
            },
            if self.0 .3 > self.1 .3 {
                self.0 .3 - (diff3 * mix_ratio) as u8
            } else {
                self.0 .3 + (diff3 * mix_ratio) as u8
            },
        );

        ret
    }
}

pub struct Size<T> {
    pub w: T,
    pub h: T,
}

impl<T> Size<T> {
    pub fn new(w: T, h: T) -> Self {
        Self { w, h }
    }
}
