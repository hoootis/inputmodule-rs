use heapless::Vec;
use crate::addon;
use crate::addon::vector2::Vector2;
use crate::matrix::{Grid, LedmatrixState, HEIGHT, WIDTH};

pub mod vector2;

pub struct VisualKeypress {
    pub keycode: u16,
    pub life: u8,
    pub alive: bool,
}

pub enum AddonAnimation {
    Spiral,
    Splashes,
}
#[derive(num_derive::FromPrimitive)]
pub enum AddonAnimationVals {
    Spiral = 0x00,
    Splashes = 0x01,
}

pub fn draw_addon_animation(state: &LedmatrixState, addon_animation: &AddonAnimation) -> Grid {
    let mut grid = Grid::default();

    let aspect_ratio: f32 = (WIDTH as f32) / (HEIGHT as f32);
    let time = state.timer as f32;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let xnorm: f32 = ((8 - x) as f32 + 0.5) / (WIDTH - 1) as f32;
            let ynorm: f32 = (y as f32 + 0.5) / (HEIGHT - 1) as f32;
            let uv = Vector2::new(xnorm, ynorm);
            let uv_centered = Vector2::new((xnorm - 0.5) * 2.0, ((ynorm - 0.5) / aspect_ratio) * 2.0);
            let new_val: f32 = match addon_animation {
                AddonAnimation::Spiral => spiral(state, uv, uv_centered, time),
                AddonAnimation::Splashes => splashes(state, uv, uv_centered, time),
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
        if (rand(keypress.keycode.wrapping_add(100) as u32) < 0.5) == state.side.is_left() { continue; }
        let mut p = uv_centered;
        p.y += rand(keypress.keycode as u32) * 6.0 - 3.0;
        p.x += rand(keypress.keycode.wrapping_add(50) as u32) - 0.5;
        let len = uv_centered.length();

        let life = keypress.life as f32 / 25.0;
        let rad = life * 2.0;

        ret = ret.max(libm::fabsf(libm::sinf(len * 3.0 - life - (time) * 0.1)) * (rad - len));
    }
    ret
}

pub fn rand(seed: u32) -> f32
{
    let b = 32;
    let f = f32::MANTISSA_DIGITS - 1;
    f32::from_bits((1 << (b - 2)) - (1 << f) + (gen_u64(seed as u64) as u32 >> (b - f))) - 1.0
}

fn gen_u64(seed: u64) -> u64 {
    // ripped from fastrand
    // Constants for WyRand taken from: https://github.com/wangyi-fudan/wyhash/blob/master/wyhash.h#L151
    // Updated for the final v4.2 implementation with improved constants for better entropy output.
    const WY_CONST_0: u64 = 0x2d35_8dcc_aa6c_78a5;
    const WY_CONST_1: u64 = 0x8bb8_4b93_962e_acc9;

    let s = seed.wrapping_add(WY_CONST_0);
    let t = u128::from(s) * u128::from(s ^ WY_CONST_1);
    (t as u64) ^ (t >> 64) as u64
}