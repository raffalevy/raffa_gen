//! Learning about RGB blending:
//! how to calculate the color of shapes with <100% opacity.

use cairo;
use cairo::{Context, ImageSurface, Surface};
use calc_rgb;
use cocoa::foundation::NSPoint;
use lerp_rgb;
use libc::c_void;
use old_rand;
use gen_rand_f64;
use rgb_to_f64;

/// Draw a rectangle to the given RGB24/ARGB32 pixel buffer.
///
/// # Safety
///
/// Unsafe because it dereferences a raw pointer and DOES NOT PERFORM BOUNDS CHECKING ON THE RECTANGLE DIMENSIONS.
pub unsafe fn rect(buf: *mut u32, x: usize, y: usize, width: usize, height: usize, color: u32) {
    let mut yi = y;
    let mut bufyi = buf.add(1000 * y);
    while yi < y + height {
        let mut xi = x;
        let mut bufi = bufyi.add(x);
        while xi < x + width {
            *bufi = color;
            xi += 1;
            bufi = bufi.offset(1);
        }
        yi += 1;
        bufyi = bufyi.offset(1000);
    }
}

/// Draw a rectangle to the given RGB24 pixel buffer, with a given opacity (NOT PREMULTIPLIED).
///
/// # Safety
///
/// Unsafe because it dereferences a raw pointer and DOES NOT PERFORM BOUNDS CHECKING ON THE RECTANGLE DIMENSIONS.
pub unsafe fn rect_with_opacity(
    buf: *mut u32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    rgb: (f64, f64, f64),
    opacity: f64,
) {
    let mut yi = y;
    let mut bufyi = buf.add(1000 * y);
    while yi < y + height {
        let mut xi = x;
        let mut bufi = bufyi.add(x);
        while xi < x + width {
            let new_rgb = lerp_rgb(rgb_to_f64(*bufi), rgb, opacity);
            *bufi = calc_rgb(new_rgb.0, new_rgb.1, new_rgb.2);
            xi += 1;
            bufi = bufi.offset(1);
        }
        yi += 1;
        bufyi = bufyi.offset(1000);
    }
}

/// Draw a rectangle to the given RGB24 pixel buffer, with a given opacity (NOT PREMULTIPLIED).
/// Instead of using normal blending, uses a dissolve blending method.
///
/// # Safety
///
/// Unsafe because it dereferences a raw pointer and DOES NOT PERFORM BOUNDS CHECKING ON THE RECTANGLE DIMENSIONS.
pub unsafe fn rect_with_opacity_dissolve(
    buf: *mut u32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
    opacity: f64,
) {
    let mut yi = y;
    let mut bufyi = buf.add(1000 * y);
    while yi < y + height {
        let mut xi = x;
        let mut bufi = bufyi.add(x);
        while xi < x + width {
            if gen_rand_f64() < opacity {
                *bufi = color;
            }
            xi += 1;
            bufi = bufi.offset(1);
        }
        yi += 1;
        bufyi = bufyi.offset(1000);
    }
}

/// Drawing callback for the opacity test
pub extern "C" fn draw(ctx: *mut c_void, _mouse: NSPoint) -> () {
    // Create the CGContext Cairo surface
    let s = Surface::quartz_create_for_cg_context(ctx, 500, 500);

    // Create a new Cairo context from the surface
    let c = Context::new(&s);

    // Make an ImageSurface on which to draw. 1000x1000 because of retina resolution.
    let mut is = ImageSurface::create(cairo::Format::Rgb24, 1000, 1000).unwrap();
    // Braces to limit the lifetime of buf
    {
        // Get the raw buffer.
        let mut buf = is.get_data().unwrap().as_mut_ptr() as *mut u32;
        // Draw some rectangles with varying opacities and blend modes
        unsafe { rect(buf, 0, 0, 700, 700, 0x00ffffff) };
        unsafe { rect(buf, 100, 100, 400, 400, 0x00ff0000) };
        unsafe { rect_with_opacity(buf, 200, 150, 600, 320, (0.0, 0.0, 1.0), 0.5) };
        unsafe { rect_with_opacity(buf, 300, 50, 70, 900, (1.0,1.0,0.0), 0.25) };
        unsafe { rect_with_opacity(buf, 350, 50, 70, 900, (1.0,1.0,0.0), 0.75) };
        unsafe { rect_with_opacity_dissolve(buf, 400, 400, 200, 400, 0x00ff00ff, 0.75)};
        unsafe { rect_with_opacity_dissolve(buf, 600, 400, 200, 400, 0x00ff00ff, 0.25)};
        unsafe { rect_with_opacity_dissolve(buf, 500, 200, 200, 400, 0x00ff0000, 0.5)};
    }

    // Save the current transformation matrix
    let m = c.get_matrix();
    // Scale the transformation matrix to accomodate for retina
    c.scale(0.5, 0.5);
    // Set the drawn-on ImageSurface as the drawing source
    c.set_source_surface(&is, 0.0, 0.0);
    // Reset the transformation matrix
    c.set_matrix(m);

    // Draw the contents of the ImageSurface
    c.rectangle(0.0, 0.0, 500.0, 500.0);
    c.fill();
}
