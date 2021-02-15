use anyhow;
use clap::Clap;
use image::RgbImage;
use indicatif::{ProgressBar, ProgressIterator};
use raytracer::{Colour, Ray, Vec3};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Eric Langlois")]
struct Opts {
    #[clap(short, long, default_value = "images/image.png")]
    output: String,

    // Defaults to 16:9 aspect ratio
    #[clap(long, default_value = "400")]
    width: u32,

    #[clap(long, default_value = "225")]
    height: u32,
}

fn ray_colour(ray: &Ray<f64>) -> Colour {
    // A simple gradient
    let unit_direction = ray.direction.unit_vector();
    let t = (unit_direction.y + 1.0) / 2.0;
    Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
}

fn main() -> Result<(), anyhow::Error> {
    let opts: Opts = Opts::parse();

    // Image
    let mut image = RgbImage::new(opts.width, opts.height);
    let image_width = image.width();
    let image_height = image.height();

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height / image_height as f64 * image_width as f64;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("Rendering...");
    let bar = ProgressBar::new((image_height * image_width) as u64);
    bar.set_draw_delta(5000);
    for (x, y, pixel) in image.enumerate_pixels_mut().progress_with(bar) {
        // (u, v) measure from bottom left corner
        let u = (x as f64) / ((image_width - 1) as f64);
        let v = ((image_height - 1 - y) as f64) / ((image_height - 1) as f64);
        // Ray from the origin to this point
        let ray = Ray {
            origin,
            direction: lower_left_corner + horizontal * u + vertical * v - origin,
        };
        *pixel = ray_colour(&ray).into();
    }

    println!("Saving image to '{}'", opts.output);
    image.save(opts.output)?;
    Ok(())
}
