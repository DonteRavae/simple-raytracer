use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    // Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for height in (0..image_height).map(|x| x as f32) {
        log::info!("Scanlines remaining: {}", (image_height as f32) - height);
        for width in (0..image_width).map(|x| x as f32) {
            println!("");
            let red = width / ((image_width - 1) as f32);
            let green = height / ((image_height - 1) as f32);
            let blue: f64 = 0.0;

            let ir = (255.999 * red) as i64;
            let ig = (255.999 * green) as i64;
            let ib = (255.999 * blue) as i64;

            println!("{ir} {ig} {ib}");
        }
    }

    log::info!("Done.")
}
