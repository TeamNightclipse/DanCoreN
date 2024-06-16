#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct ColorHex(pub i32);

impl ColorHex {
    pub fn to_rgb(self) -> ColorRgb {
        ColorRgb {
            r: ((self.0 >> 16) & 0xFF) as u8,
            g: ((self.0 >> 8) & 0xFF) as u8,
            b: (self.0 & 0xFF) as u8,
        }
    }
    
    pub fn lerp_through_hsv(self, other: ColorHex, t: f32) -> ColorHex {
        self.to_rgb().to_hsv().lerp(&other.to_rgb().to_hsv(), t).to_rgb().to_hex()
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct ColorRgb {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorRgb {
    //https://stackoverflow.com/questions/3018313/algorithm-to-convert-rgb-to-hsv-and-hsv-to-rgb-in-range-0-255-for-both
    pub fn to_hsv(&self) -> ColorHsv {
        let ColorRgb { r, g, b } = *self;
        let rd = r as f32 / 255.0;
        let gd = g as f32 / 255.0;
        let bd = b as f32 / 255.0;

        let min = rd.min(gd.min(bd));
        let max = rd.max(gd.max(bd));

        let delta = max - min;
        if delta < 0.00001 || max <= 0.0 {
            ColorHsv {
                h: 0.0, //We just choose a value to not emit NaN if max <= 0.0
                s: 0.0,
                v: max,
            }
        } else {
            let sat = delta / max;

            let mut hue = if rd >= max {
                (gd - bd) / delta
            } else if gd >= max {
                2.0 + (bd - rd) / delta
            } else {
                4.0 + (rd - gd) / delta
            };

            hue *= 60.0;

            if hue < 0.0 {
                hue += 360.0;
            }

            ColorHsv {
                h: hue,
                s: sat,
                v: max,
            }
        }
    }

    pub fn to_hex(&self) -> ColorHex {
        ColorHex(((self.r as i32) << 16) | ((self.g as i32) << 8) | (self.b as i32))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ColorHsv {
    h: f32,
    s: f32,
    v: f32,
}

impl ColorHsv {
    pub fn to_rgb(&self) -> ColorRgb {
        let ColorHsv { mut h, s, v } = *self;

        if s <= 0.0 {
            return ColorRgb { r: 0, g: 0, b: 0 };
        }

        if h >= 360.0 {
            h = 0.0;
        }

        h /= 60.0;

        let i = h as u8;
        let ff = h - i as f32;

        let p = ((v * (1.0 - s)) * 255.0) as u8;
        let q = ((v * (1.0 - (s * ff))) * 255.0) as u8;
        let t = ((v * (1.0 - (s * (1.0 - ff)))) * 255.0) as u8;

        let vv = (v * 255.0) as u8;

        match i {
            0 => ColorRgb { r: vv, g: t, b: p },
            1 => ColorRgb { r: q, g: vv, b: p },
            2 => ColorRgb { r: p, g: vv, b: t },
            3 => ColorRgb { r: p, g: q, b: vv },
            4 => ColorRgb { r: t, g: p, b: vv },
            _ => ColorRgb { r: vv, g: p, b: q },
        }
    }

    // https://www.alanzucconi.com/2016/01/06/colour-interpolation/
    pub fn lerp(&self, that: &ColorHsv, mut t: f32) -> ColorHsv {
        // Hue interpolation
        let mut d = that.h - self.h;

        let mut ah = self.h;
        let mut bh = that.h;

        if self.h > that.h {
            // Swap (a.h, b.h)
            std::mem::swap(&mut bh, &mut ah);
            d = -d;
            t = 1.0 - t;
        }

        let h = if d > 0.5 {
            // 180deg
            ah += 1.0; // 360deg
            (ah + t * (bh - ah)) % 1.0 // 360deg
        } else {
            // 180deg
            ah + t * d
        };
        // Interpolates the rest

        ColorHsv {
            h,
            s: nalgebra_glm::lerp_scalar(self.s, that.s, t),
            v: nalgebra_glm::lerp_scalar(self.v, that.v, t),
        }
    }
}
