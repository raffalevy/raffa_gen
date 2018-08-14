use lerp;
use clip;
use rgb_to_f64;
use calc_rgb;

#[test]
fn test_clip() {
    assert_eq!(clip(10.0), 1.0);
    assert_eq!(clip(2.0), 1.0);
    assert_eq!(clip(1.0), 1.0);
    assert_eq!(clip(0.9), 0.9);
    assert_eq!(clip(0.5), 0.5);
    assert_eq!(clip(0.1), 0.1);
    assert_eq!(clip(0.0), 0.0);
    assert_eq!(clip(-0.5), 0.0);
    assert_eq!(clip(-1.0), 0.0);
    assert_eq!(clip(-100.0), 0.0);
}

#[test]
fn test_calc_rgb() {
    assert_eq!(calc_rgb(1.0, 1.0, 0.0), 0x00ffff00);
    assert_eq!(calc_rgb(0.0, 0.0, 0.0), 0x00000000);
    assert_eq!(calc_rgb(1.0, 1.0, 1.0), 0x00ffffff);
}

#[test]
fn test_rgb_to_f64() {
    assert_eq!((1.0, 1.0, 0.0), rgb_to_f64(0x00ffff00));
    assert_eq!((0.0, 0.0, 0.0), rgb_to_f64(0x00000000));
    assert_eq!((1.0, 1.0, 1.0), rgb_to_f64(0x00ffffff));
}

#[test]
fn test_lerp() {
    assert_eq!(lerp(0.0, 1.0, 0.25), 0.25);
    assert_eq!(lerp(1.0, 0.0, 0.75), 0.25);
    assert_eq!(lerp(0.0, 0.0, 0.532), 0.0);
    assert_eq!(lerp(52.0, 52.0, 0.532), 52.0);
    assert_eq!(lerp(9.0, 18.0, 1.5), 22.5);
    assert_eq!(lerp(0.0, 2.0, -2.0), -4.0);
}