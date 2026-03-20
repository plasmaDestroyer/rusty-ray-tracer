mod color;
mod vec3;

use color::Color;
use color::write_color;
use std::io::Write;

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
            let pixel_color = Color::new(
                (j as f64) / ((image_width - 1) as f64),
                (i as f64) / ((image_height - 1) as f64),
                0.0,
            );
            write_color(&mut std::io::stdout(), &pixel_color);
        }
    }

    eprintln!("\rDone.                 ");
}
