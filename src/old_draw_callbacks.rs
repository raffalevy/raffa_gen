use cairo;
use cairo::{Context, ImageSurface, Surface};
use calc_rgb;
use cocoa::foundation::NSPoint;
use fake_rand_u64;
use gen_rand_f64;
use libc::c_void;
use old_rand;
use std;
use std::f64::consts::PI;

static mut t: f64 = 0.0;

unsafe extern "C" fn draw_scribble(ctx: *mut c_void, mouse: NSPoint) -> () {
    old_rand = fake_rand_u64(old_rand.wrapping_add(mouse.x as u64));
    old_rand = fake_rand_u64(old_rand.wrapping_add(mouse.y as u64));
    let s = Surface::quartz_create_for_cg_context(ctx, 500, 500);
    let c = Context::new(&s);
    // println!("{:08x}", ((clip(1.0) * 255.0) as u32) << 4);
    // println!("{:08x}", calc_rgb(1.0,1.0,0.0));
    let mut is = ImageSurface::create(cairo::Format::Rgb24, 1000, 1000).unwrap();
    {
        let mut buf = is.get_data().unwrap().as_mut_ptr() as *mut u32;
        let mut y = 0.0;
        while y < 1000.0 {
            let mut x = 0.0;
            while x < 1000.0 {
                let dx = x - 500.0;
                let dy = y - 500.0;
                let r = f64::sqrt(dx * dx + dy * dy) / 4.0;
                let a = f64::atan2(dy, dx);
                // *buf = calc_rgb(f64::sin(r/10.0)/2.0+0.5*f64::cos(a*10.0)/2.0+0.5, f64::sin(r/13.0)/2.0+0.5*f64::cos(a*17.0)/2.0+0.5, 0.5);
                *buf = calc_rgb((r % 50.0) / 50.0, (r % 100.0) / 100.0, (r % 200.0) / 200.0);
                x += 1.0;
                buf = buf.wrapping_offset(1);
            }
            y += 1.0;
        }
    }
    c.set_source_rgb(1.0, 1.0, 1.0);
    c.rectangle(0.0, 0.0, 500.0, 500.0);
    c.fill();
    let m = c.get_matrix();
    c.scale(0.5, 0.5);
    c.set_source_surface(&is, 0.0, 0.0);
    c.set_matrix(m);
    // c.rectangle(0., 0., 500., 500.);
    c.set_line_width(15.0);

    // c.move_to(0.0, 0.0);
    // c.curve_to(0.0, 500.0, 500.0, 500.0, 500.0, 0.0);
    // c.stroke();

    // c.move_to(0.0, 500.0);
    // c.curve_to(0.0, 0.0, 500.0, 0.0, 500.0, 500.0);
    // c.stroke();

    // c.move_to(0.0, 0.0);
    // c.curve_to(500.0, 0.0, 500.0, 500.0, 0.0, 500.0);
    // c.stroke();

    // c.move_to(500.0, 0.0);
    // c.curve_to(0.0, 0.0, 0.0, 500.0, 500.0, 500.0);
    // c.stroke();

    // c.arc(250.0, 250.0, 220.0, 0.0, 2.0 * PI);
    // c.stroke();

    // c.arc(250.0, 250.0, 75.0, 0.0, 2.0 * PI);
    // c.stroke();

    // c.arc(250.0, 250.0, 50.0, 0.0, 2.0 * PI);
    // c.stroke();

    // c.arc(250.0, 250.0, 25.0, 0.0, 2.0 * PI);
    // c.stroke();

    // c.arc(250.0, 250.0, 7.5, 0.0, 2.0 * PI);
    // c.fill();

    c.move_to(gen_rand_f64() * 500.0, gen_rand_f64() * 500.0);
    // c.line_to(gen_rand_f64() * 500.0, gen_rand_f64() * 500.0);
    // c.line_to(gen_rand_f64() * 500.0, gen_rand_f64() * 500.0);
    for _ in 0..10 {
        c.curve_to(
            gen_rand_f64() * 500.0,
            gen_rand_f64() * 500.0,
            gen_rand_f64() * 500.0,
            gen_rand_f64() * 500.0,
            gen_rand_f64() * 500.0,
            gen_rand_f64() * 500.0,
        );
    }

    c.stroke();

    // c.rectangle(0.0, 0.0, 500.0, 500.0);
    // c.fill();

    // c.fill();

    t += 1.0;
}

unsafe extern "C" fn draw_bullseye(ctx: *mut c_void, mouse: NSPoint) -> () {
    old_rand = fake_rand_u64(old_rand.wrapping_add(mouse.x as u64));
    old_rand = fake_rand_u64(old_rand.wrapping_add(mouse.y as u64));
    let s = Surface::quartz_create_for_cg_context(ctx, 500, 500);
    let c = Context::new(&s);
    // c.set_source_rgb(0.0, 0.0, 1.0);
    // c.move_to(0.0, 0.0);
    // c.line_to(mouse.x, mouse.y);
    // c.stroke();
    // #[allow(non_snake_case)]
    // let (mouseXi, mouseYi) = (mouse.x as usize, mouse.y as usize);
    let mut is = ImageSurface::create(cairo::Format::Rgb24, 500, 500).unwrap();
    {
        let mut buf = is.get_data().unwrap().as_mut_ptr() as *mut u32;
        let mut x = 0.0;
        while x < 500.0 {
            let mut y = 0.0;
            while y < 500.0 {
                let dx = x - 250.0;
                let dx = dx * 1.5;
                let dy = y - 250.0;
                let r = f64::sqrt(dx * dx + dy * dy);
                let a = f64::atan2(dy, dx);
                old_rand = fake_rand_u64(old_rand);
                // let cond = old_rand % 262144 < (x * y) as u64;
                // 3.1830988618
                let cond = (old_rand as f64) / (std::u64::MAX as f64)
                    < f64::sin(r / 7.9577471545) * f64::sin(a * 13.0);
                if cond {
                    *buf = 0x00ffffff;
                }
                y += 1.0;
                buf = buf.wrapping_offset(1);
            }
            x += 1.0;
        }
    }
    c.set_source_surface(&is, 0.0, 0.0);
    // c.rectangle(0., 0., 500., 500.);
    // c.move_to(0.0, 0.0);
    // c.curve_to(100.0, 400.0, 400.0, 100.0, 500.0, 0.0);
    // c.curve_to(0.0, 0.0, 0.0, 500.0, 500.0, 500.0);

    c.set_line_width(10.0);
    let mut r = 20.0;
    while r <= 340.0 {
        c.arc(250.0, 250.0, r, 0.0, 2.0 * PI);
        c.stroke();
        r += 20.0
    }
    c.arc(250.0, 250.0, 5.0, 0.0, 2.0 * PI);

    c.fill();
}
