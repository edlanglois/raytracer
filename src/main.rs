use anyhow;
use clap::Clap;
use image::RgbImage;
use indicatif::{ProgressBar, ProgressIterator};
use raytracer::objects::Sphere;
use raytracer::{Colour, Ray, Surface, Vec3};
use std::f64;

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

fn ray_colour<T: Surface>(ray: &Ray<f64>, surface: &T) -> Colour {
    if let Some(intersection) = surface.intersect(ray, 0.0, f64::INFINITY) {
        let n = intersection.normal;
        return Colour::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) / 2.0;
    }

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

    // World
    let mut world: Vec<Box<dyn Surface>> = Vec::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))); // ground
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));

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
        *pixel = ray_colour(&ray, &world).into();
    }

    println!("Saving image to '{}'", opts.output);
    image.save(opts.output)?;
    Ok(())
}
