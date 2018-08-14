#![allow(non_upper_case_globals, dead_code)]
//! My attempt to make fast-rendering generative art in Rust.
//! Also just a playground project for me to learn about 2D graphics.
//! Currently only runs on MacOS using a Quartz backend.

extern crate cocoa;
use cocoa::foundation::NSPoint;
extern crate cairo;
extern crate libc;
use libc::c_void;

mod old_draw_callbacks;
pub mod opa;

#[cfg(test)]
mod tests;

extern "C" {
    /// The Objective-C setup function. Creates a window in which to draw and basic UI.
    /// Takes a Rust drawing callback which will be used to render the contents of the window.
    pub fn setup(drawcb: Option<extern "C" fn(*mut c_void, NSPoint) -> ()>);
}

pub fn main() {
    unsafe {
        // Call the Objective-C setup function with a drawing callback
        setup(Some(opa::draw));
    }
}

/// Pseudo-random number generator. No semblance of cryptographic security is intended.
pub fn fake_rand_u64(mut seed: u64) -> u64 {
    for _ in 0..32 {
        seed = seed.wrapping_add(1);
        // Arbitrary number relatively prime to word size (2^64)
        seed = seed.wrapping_mul(0x83b17bd9484e1e13);
        seed = seed.wrapping_add(1);
        seed = seed.rotate_right(5);
    }
    seed
}

const MAX64: f64 = std::u64::MAX as f64;

#[inline]
pub fn rand_to_f64(r: u64) -> f64 {
    (r as f64) / MAX64
}

#[inline]
pub unsafe fn gen_rand_f64() -> f64 {
    old_rand = fake_rand_u64(old_rand);
    rand_to_f64(old_rand)
}

/// The current random seed
pub static mut old_rand: u64 = 0xd572fbe35626507a;

/// Clip an `f64` to the range [0.0, 1.0]
#[inline]
pub fn clip(x: f64) -> f64 {
    if x > 1.0 {
        1.0
    } else if x < 0.0 {
        0.0
    } else {
        x
    }
}

/// Calculate a 24-bit (32-bit-aligned) RGB value from floats. Inverse of [`rgb_to_f64`].
#[inline]
pub fn calc_rgb(r: f64, g: f64, b: f64) -> u32 {
    let rv = (clip(r) * 255.0) as u32;
    let gv = (clip(g) * 255.0) as u32;
    let bv = (clip(b) * 255.0) as u32;
    (rv << 16) + (gv << 8) + bv
}

/// Inverse of [`calc_rgb`]; calculate floats from a 24-bit RGB value.
#[inline]
pub fn rgb_to_f64(val: u32) -> (f64, f64, f64) {
    (
        ((val & 0x00ff0000) >> 16) as f64 / 255.0,
        ((val & 0x0000ff00) >> 8) as f64 / 255.0,
        (val & 0x000000ff) as f64 / 255.0,
    )
}

/// Linear interpolation between a and b.
/// HIGHER WEIGHTS MEAN CLOSER TO B.
#[inline]
pub fn lerp(a: f64, b: f64, weight: f64) -> f64 {
    b * weight + a * (1.0 - weight)
}

/// Run [`lerp`] on each value in an RGB tuple
#[inline]
pub fn lerp_rgb(a: (f64, f64, f64), b: (f64, f64, f64), weight: f64) -> (f64, f64, f64) {
    (
        lerp(a.0, b.0, weight),
        lerp(a.1, b.1, weight),
        lerp(a.2, b.2, weight),
    )
}
