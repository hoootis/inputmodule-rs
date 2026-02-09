use crate::addon::vector2::Vector2;
use crate::matrix::{Grid, LedmatrixState, Side, HEIGHT, WIDTH};
use core::f32::consts::PI;

pub mod vector2;

pub struct VisualKeypress {
    pub keycode: u32,
    pub life: u8,
    pub alive: bool,
    pub side: Side,
    pub rand0: f32,
    pub rand1: f32,
}

pub enum AddonAnimation {
    Spiral,
    Splashes,
    Helix,
}
#[derive(num_derive::FromPrimitive)]
pub enum AddonAnimationVals {
    Spiral = 0x00,
    Splashes = 0x01,
    Helix = 0x02,
}

#[derive(Copy, Clone)]
pub struct CachedUV {
    pub uv: Vector2,
    pub uv_centered: Vector2,
}
pub const CACHED_UVS: [[CachedUV; 34]; 9] = {
    let mut result = [[CachedUV { uv: Vector2::default(), uv_centered: Vector2::default() }; 34]; 9];

    const ASPECT_RATIO: f32 = (WIDTH as f32) / (HEIGHT as f32);
    let mut x = 0;
    while x < WIDTH {
        let mut y = 0;
        while y < HEIGHT {
            let xnorm: f32 = ((WIDTH - 1 - x) as f32) / (WIDTH - 1) as f32;
            let ynorm: f32 = (y as f32 + 0.5) / (HEIGHT - 1) as f32;
            result[x][y] = CachedUV { uv: Vector2::new(xnorm, ynorm), uv_centered: Vector2::new((xnorm - 0.5) * 2.0, ((ynorm - 0.5) / ASPECT_RATIO) * 2.0) };

            y += 1;
        }
        x += 1;
    }

    result
};

pub fn draw_addon_animation(state: &LedmatrixState, addon_animation: &AddonAnimation) -> Grid {
    let mut grid = Grid::default();

    let time = state.timer as f32;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let new_val: f32 = match addon_animation {
                AddonAnimation::Spiral => spiral(state, CACHED_UVS[x][y].uv, CACHED_UVS[x][y].uv_centered, time),
                AddonAnimation::Splashes => splashes(state, CACHED_UVS[x][y].uv, CACHED_UVS[x][y].uv_centered, time),
                AddonAnimation::Helix => helix(state, CACHED_UVS[x][y].uv, CACHED_UVS[x][y].uv_centered, time),
            };

            let new_val = new_val.clamp(0.0, 1.0);
            let new_val = new_val * new_val; // brightness preception is non-linear; this makes it look linear
            grid.0[x][y] = (new_val * 255.0) as u8;
        }
    }

    grid
}

pub fn spiral(state: &LedmatrixState, uv: Vector2, uv_centered: Vector2, time: f32) -> f32 {
    const RAD: f32 = 5.0;
    let len = uv_centered.length();
    let angle = libm::atan2f(uv_centered.y, uv_centered.x);
    libm::sinf(angle + len * RAD - time * 0.1)
}

pub fn splashes(state: &LedmatrixState, uv: Vector2, uv_centered: Vector2, time: f32) -> f32 {
    let mut ret: f32 = 0.0;
    for keypress in state.visual_keypresses.iter()
    {
        if keypress.side != state.side { continue; }
        let mut p = uv_centered;
        p.y += keypress.rand0 * 6.0 - 3.0;
        p.x += keypress.rand1 - 0.5;
        let len = p.length();

        let life = keypress.life as f32 / state.visual_keypress_life as f32;
        let rad = life * 1.5;
        if len > rad { continue; }

        ret = ret.max(f32::abs(sin(len * 2.0 - life * 5.0 - (time) * 0.1)) * (rad - len));
    }
    ret
}

pub fn helix(state: &LedmatrixState, mut uv: Vector2, uv_centered: Vector2, time: f32) -> f32 {
    uv.x -= 0.5;
    uv.x *= 2.0;

    const width: f32 = 1.5 / WIDTH as f32;
    const padding: f32 = 0.0;
    const freq: f32 = 5.0;

    let time = time * 0.1;
    let mut shade_coeff = f32::abs(uv.x) + width * 2.0;
    shade_coeff = shade_coeff * shade_coeff * shade_coeff;
    let left_offset = libm::sinf((uv.y * freq + time) + PI / 2.0) * (1.0 - width - padding);
    let left_shaded = left_offset > 0.0;

    let mut left = uv.x + libm::sinf(uv.y * freq + time) * (1.0 - width - padding);
    left = smoothstep(f32::abs(left), width * 1.5, width);
    left *= if left_shaded { shade_coeff } else { 1.0 };

    let mut right = uv.x + libm::sinf(uv.y * freq + time + PI) * (1.0 - width - padding);
    right = smoothstep(f32::abs(right), width * 1.5, width);
    right *= if left_shaded { 1.0 } else { shade_coeff };

    f32::max(left, right)
}

pub const fn step(x: f32, edge: f32) -> f32 {
    if x < edge { 1.0 } else { 0.0 }
}

pub const fn smoothstep(x: f32, edge0: f32, edge1: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[inline]
pub const fn sin(x: f32) -> f32 {
    let x = x % (PI * 2.0);
    const FOUROVERPI: f32 = 1.2732395447351627;
    const FOUROVERPISQ: f32 = 0.40528473456935109;
    const Q: f32 = 0.77633023248007499;

    let mut p = 0.22308510060189463_f32.to_bits();
    let mut v = x.to_bits();

    let sign: u32 = v & 0x80000000;
    v &= 0x7FFFFFFF;

    let approx = FOUROVERPI * x - FOUROVERPISQ * x * f32::from_bits(v);

    p |= sign;

    approx * (Q + f32::from_bits(p) * approx)
}

#[inline(always)]
pub fn rand(mut x: u32) -> f32 {
    x = x.wrapping_mul(0x9E37_79B9);

    // I LOVE XORSHIFT
    x ^= x >> 16;
    x = x.wrapping_mul(0x85EB_CA6B);
    x ^= x >> 13;
    x = x.wrapping_mul(0xC2B2_AE35);
    x ^= x >> 16;

    let bits = (x >> 9) | 0x3F80_0000;
    f32::from_bits(bits) - 1.0
}