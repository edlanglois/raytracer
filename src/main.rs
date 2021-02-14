use anyhow;
use clap::Clap;
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressIterator};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Eric Langlois")]
struct Opts {
    #[clap(short, long, default_value = "images/image.png")]
    output: String,

    #[clap(long, default_value = "256")]
    width: u32,

    #[clap(long, default_value = "256")]
    height: u32,
}

fn main() -> Result<(), anyhow::Error> {
    let opts: Opts = Opts::parse();
    let u8_max_float = 255.999;

    let mut image = RgbImage::new(opts.width, opts.height);

    println!("Rendering...");
    let bar = ProgressBar::new((opts.width * opts.height) as u64);
    bar.set_draw_delta(5000);
    for (i, j, pixel) in image.enumerate_pixels_mut().progress_with(bar) {
        let r = (i as f64) / ((opts.height - 1) as f64);
        let g = (j as f64) / ((opts.width - 1) as f64);
        let b = 0.25f64;

        let ir = (u8_max_float * r) as u8;
        let ig = (u8_max_float * g) as u8;
        let ib = (u8_max_float * b) as u8;
        *pixel = Rgb([ir, ig, ib]);
    }

    println!("Saving image to '{}'", opts.output);
    image.save(opts.output)?;
    Ok(())
}
