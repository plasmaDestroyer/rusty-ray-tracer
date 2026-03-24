use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl std::io::Write, pixel_color: &Color) {
    let r: f64 = pixel_color.x();
    let g: f64 = pixel_color.y();
    let b: f64 = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::new(0.000, 0.999);
    let rbyte: u32 = (256.0 * intensity.clamp(r)) as u32;
    let gbyte: u32 = (256.0 * intensity.clamp(g)) as u32;
    let bbyte: u32 = (256.0 * intensity.clamp(b)) as u32;

    let _ = writeln!(out, "{} {} {}", rbyte, gbyte, bbyte);
}
