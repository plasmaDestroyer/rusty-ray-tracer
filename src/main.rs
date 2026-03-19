mod vec3;

use std::io::Write;
use vec3::Vec3;

fn main() {
    // Image

    let image_width: u32 = 256;
    let image_height: u32 = 256;

    //Render

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for i in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - i);
        std::io::stderr().flush().unwrap();
        for j in 0..image_width {
            let r: f64 = (j as f64) / ((image_width - 1) as f64);
            let g: f64 = (i as f64) / ((image_height - 1) as f64);
            let b: f64 = 0.0;

            let ir: u32 = (r * 255.999) as u32;
            let ig: u32 = (g * 255.999) as u32;
            let ib: u32 = (b * 255.999) as u32;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }

    eprintln!("\rDone.                 ");
}
