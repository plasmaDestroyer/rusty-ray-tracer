use crate::interval::Interval;
use crate::vec3::Vec3;
use image::Rgb;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn to_rgb(pixel_color: &Color) -> Rgb<u8> {
    let mut r: f64 = pixel_color.x();
    let mut g: f64 = pixel_color.y();
    let mut b: f64 = pixel_color.z();

    // Apply a linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::new(0.000, 0.999);
    let rbyte: u8 = (256.0 * intensity.clamp(r)) as u8;
    let gbyte: u8 = (256.0 * intensity.clamp(g)) as u8;
    let bbyte: u8 = (256.0 * intensity.clamp(b)) as u8;

    Rgb([rbyte, gbyte, bbyte])
}
