//! Minimal big-endian reader/writer for Qt's `QDataStream`
//! (stream version `Qt_4_5`), covering exactly the types `.spf` files use:
//! integers, doubles, bool, `QString`, `QColor`, `QPointF`, `QPolygonF`
//! and `QFont`.
//!
//! Like `QDataStream`, reading past the end does not panic: the reader
//! enters a sticky "overrun" state and every further read yields zero /
//! empty values. Callers check [`Reader::overrun`] at section boundaries.

pub type PointF = (f64, f64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }
}

/// A `QFont` as stored in a Qt_4_5 data stream.
///
/// Layout (verified against the sample files shipped with sconcho):
/// family `QString`, point size `f64`, pixel size `i32`, then 16 bytes
/// (style hint, style strategy, charset, weight, font bits, stretch u16,
/// further flags, letter spacing). The 16 bytes are kept verbatim so that a
/// load/save cycle never alters them.
#[derive(Debug, Clone, PartialEq)]
pub struct Font {
    pub family: String,
    pub point_size: f64,
    pub pixel_size: i32,
    pub tail: [u8; 16],
}

impl Font {
    /// Tail bytes of a plain, normal-weight font as written by Qt 4.
    pub const DEFAULT_TAIL: [u8; 16] =
        [5, 1, 0, 50, 0x10, 0, 100, 1, 0, 0, 0, 0, 0, 0, 0, 0];

    pub fn new(family: &str, point_size: f64) -> Self {
        Font {
            family: family.to_string(),
            point_size,
            pixel_size: -1,
            tail: Self::DEFAULT_TAIL,
        }
    }

    /// Qt 4 weight scale: 50 = normal, 75 = bold.
    pub fn weight(&self) -> u8 {
        self.tail[3]
    }
    pub fn italic(&self) -> bool {
        self.tail[4] & 0x01 != 0
    }
    pub fn underline(&self) -> bool {
        self.tail[4] & 0x02 != 0
    }
}

const SPEC_RGB: i8 = 1;
const SPEC_HSV: i8 = 2;
const SPEC_CMYK: i8 = 3;
const SPEC_HSL: i8 = 4;

fn to8(v: f64) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// `h` in degrees, `s`/`v` in 0..=1.
fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c = v * s;
    let hp = h.rem_euclid(360.0) / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r1, g1, b1) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    (to8(r1 + m), to8(g1 + m), to8(b1 + m))
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h.rem_euclid(360.0) / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r1, g1, b1) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    (to8(r1 + m), to8(g1 + m), to8(b1 + m))
}

pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
    overrun: bool,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Reader { buf, pos: 0, overrun: false }
    }

    pub fn at_end(&self) -> bool {
        self.pos >= self.buf.len()
    }

    pub fn overrun(&self) -> bool {
        self.overrun
    }

    /// Forget a previous overrun (used to tolerate truncated optional tails).
    pub fn clear_overrun(&mut self) {
        self.overrun = false;
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.pos)
    }

    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.overrun || self.remaining() < n {
            self.overrun = true;
            return None;
        }
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Some(s)
    }

    pub fn u8(&mut self) -> u8 {
        self.take(1).map_or(0, |b| b[0])
    }
    pub fn i8(&mut self) -> i8 {
        self.u8() as i8
    }
    pub fn bool(&mut self) -> bool {
        self.u8() != 0
    }
    pub fn u16(&mut self) -> u16 {
        self.take(2).map_or(0, |b| u16::from_be_bytes([b[0], b[1]]))
    }
    pub fn i16(&mut self) -> i16 {
        self.take(2).map_or(0, |b| i16::from_be_bytes([b[0], b[1]]))
    }
    pub fn u32(&mut self) -> u32 {
        self.take(4)
            .map_or(0, |b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }
    pub fn i32(&mut self) -> i32 {
        self.take(4)
            .map_or(0, |b| i32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }
    pub fn f64(&mut self) -> f64 {
        self.take(8)
            .map_or(0.0, |b| f64::from_be_bytes(<[u8; 8]>::try_from(b).unwrap()))
    }

    pub fn skip_i32(&mut self, count: usize) {
        for _ in 0..count {
            self.i32();
        }
    }

    /// `QString`: u32 byte length (0xFFFFFFFF = null) followed by UTF-16BE.
    pub fn qstring(&mut self) -> String {
        let len = self.u32();
        if self.overrun || len == 0xFFFF_FFFF {
            return String::new();
        }
        let len = len as usize;
        if len % 2 != 0 {
            self.overrun = true;
            return String::new();
        }
        match self.take(len) {
            Some(bytes) => {
                let units: Vec<u16> = bytes
                    .chunks_exact(2)
                    .map(|c| u16::from_be_bytes([c[0], c[1]]))
                    .collect();
                String::from_utf16_lossy(&units)
            }
            None => String::new(),
        }
    }

    /// `QColor` in the Qt_4_5 format: spec `i8`, then five `u16`
    /// (alpha, three components, pad). Non-RGB specs are converted to RGB.
    pub fn color(&mut self) -> Color {
        let spec = self.i8();
        let alpha = self.u16();
        let c1 = self.u16();
        let c2 = self.u16();
        let c3 = self.u16();
        let pad = self.u16();
        let a = (alpha >> 8) as u8;
        let (r, g, b) = match spec {
            SPEC_RGB => ((c1 >> 8) as u8, (c2 >> 8) as u8, (c3 >> 8) as u8),
            // Qt stores hue as degrees * 100 in a u16.
            SPEC_HSV => hsv_to_rgb(c1 as f64 / 100.0, c2 as f64 / 65535.0, c3 as f64 / 65535.0),
            SPEC_HSL => hsl_to_rgb(c1 as f64 / 100.0, c2 as f64 / 65535.0, c3 as f64 / 65535.0),
            SPEC_CMYK => {
                let k = pad as f64 / 65535.0;
                let conv = |v: u16| to8((1.0 - v as f64 / 65535.0) * (1.0 - k));
                (conv(c1), conv(c2), conv(c3))
            }
            _ => (255, 255, 255), // invalid colour: fall back to white
        };
        Color { r, g, b, a }
    }

    pub fn point(&mut self) -> PointF {
        let x = self.f64();
        let y = self.f64();
        (x, y)
    }

    /// `QPolygonF`: u32 count followed by that many points.
    pub fn polygon(&mut self) -> Vec<PointF> {
        let n = self.u32() as usize;
        if self.overrun || n.checked_mul(16).map_or(true, |bytes| bytes > self.remaining()) {
            self.overrun = true;
            return Vec::new();
        }
        (0..n).map(|_| self.point()).collect()
    }

    pub fn font(&mut self) -> Font {
        let family = self.qstring();
        let point_size = self.f64();
        let pixel_size = self.i32();
        let mut tail = Font::DEFAULT_TAIL;
        if let Some(b) = self.take(16) {
            tail.copy_from_slice(b);
        }
        Font { family, point_size, pixel_size, tail }
    }
}

#[derive(Default)]
pub struct Writer {
    buf: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    pub fn u8(&mut self, v: u8) {
        self.buf.push(v);
    }
    pub fn i8(&mut self, v: i8) {
        self.buf.push(v as u8);
    }
    pub fn bool(&mut self, v: bool) {
        self.buf.push(v as u8);
    }
    pub fn u16(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }
    pub fn i16(&mut self, v: i16) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }
    pub fn u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }
    pub fn i32(&mut self, v: i32) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }
    pub fn f64(&mut self, v: f64) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }

    pub fn qstring(&mut self, s: &str) {
        let units: Vec<u16> = s.encode_utf16().collect();
        self.u32((units.len() * 2) as u32);
        for u in units {
            self.u16(u);
        }
    }

    pub fn color(&mut self, c: Color) {
        self.i8(SPEC_RGB);
        self.u16(c.a as u16 * 257);
        self.u16(c.r as u16 * 257);
        self.u16(c.g as u16 * 257);
        self.u16(c.b as u16 * 257);
        self.u16(0);
    }

    pub fn point(&mut self, p: PointF) {
        self.f64(p.0);
        self.f64(p.1);
    }

    pub fn polygon(&mut self, pts: &[PointF]) {
        self.u32(pts.len() as u32);
        for &p in pts {
            self.point(p);
        }
    }

    pub fn font(&mut self, f: &Font) {
        self.qstring(&f.family);
        self.f64(f.point_size);
        self.i32(f.pixel_size);
        self.buf.extend_from_slice(&f.tail);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitives_roundtrip() {
        let mut w = Writer::new();
        w.i32(-5);
        w.u16(65535);
        w.i16(-2);
        w.f64(3.25);
        w.bool(true);
        w.qstring("Zöpfe ✓");
        w.color(Color { r: 10, g: 200, b: 255, a: 255 });
        w.point((1.5, -2.5));
        w.polygon(&[(0.0, 0.0), (1.0, 2.0)]);
        w.font(&Font::new("Arial", 10.0));
        let bytes = w.into_bytes();

        let mut r = Reader::new(&bytes);
        assert_eq!(r.i32(), -5);
        assert_eq!(r.u16(), 65535);
        assert_eq!(r.i16(), -2);
        assert_eq!(r.f64(), 3.25);
        assert!(r.bool());
        assert_eq!(r.qstring(), "Zöpfe ✓");
        assert_eq!(r.color(), Color { r: 10, g: 200, b: 255, a: 255 });
        assert_eq!(r.point(), (1.5, -2.5));
        assert_eq!(r.polygon(), vec![(0.0, 0.0), (1.0, 2.0)]);
        assert_eq!(r.font(), Font::new("Arial", 10.0));
        assert!(r.at_end());
        assert!(!r.overrun());
    }

    #[test]
    fn overrun_is_sticky_and_zeroed() {
        let mut r = Reader::new(&[0, 0, 0]);
        assert_eq!(r.i32(), 0);
        assert!(r.overrun());
        assert_eq!(r.u8(), 0);
        assert_eq!(r.qstring(), "");
    }

    #[test]
    fn null_qstring_is_empty() {
        let mut r = Reader::new(&[0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(r.qstring(), "");
        assert!(!r.overrun());
    }

    #[test]
    fn qstring_wire_format() {
        let mut w = Writer::new();
        w.qstring("ab");
        assert_eq!(w.into_bytes(), vec![0, 0, 0, 4, 0, b'a', 0, b'b']);
    }
}
