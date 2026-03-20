use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl std::io::Write, pixel_color: &Color) {
    let r: f64 = pixel_color.x();
    let g: f64 = pixel_color.y();
    let b: f64 = pixel_color.z();

    let rbyte: u32 = (r * 255.999) as u32;
    let gbyte: u32 = (g * 255.999) as u32;
    let bbyte: u32 = (b * 255.999) as u32;

    let _ = writeln!(out, "{} {} {}", rbyte, gbyte, bbyte);
}
