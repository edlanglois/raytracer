use anyhow;
use clap::Clap;
use image::RgbImage;
use indicatif::{ProgressBar, ProgressIterator};
use rand;
use raytracer::colour;
use raytracer::materials::{Lambertian, Metal, Transparent};
use raytracer::objects::Sphere;
use raytracer::{Camera, Colour, Ratio, RayR3, Surface, Vec3};
use std::f64;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Eric Langlois")]
pub struct Opts {
    #[clap(short, long, default_value = "image.png")]
    pub output: String,

    #[clap(short, long, default_value = "3:2")]
    pub aspect_ratio: Ratio<u32>,

    #[clap(long, default_value = "400")]
    pub width: u32,

    #[clap(short, long, default_value = "100")]
    pub samples_per_pixel: u32,

    #[clap(long, default_value = "50")]
    pub max_depth: u32,
}

fn main() -> Result<(), anyhow::Error> {
    let opts: Opts = Opts::parse();
    let image_width = opts.width;
    let image_height = opts.aspect_ratio.a_to_b(image_width);
    println!("Image dimensions: {} x {}", image_width, image_height);

    // Image
    let mut image = RgbImage::new(image_width, image_height);

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    // let distance_to_focus = (lookfrom - lookat).norm();
    let distance_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        opts.aspect_ratio.into(),
        aperture,
        distance_to_focus,
    );

    // Render
    println!("Rendering...");
    let bar = ProgressBar::new((image_height * image_width) as u64);
    bar.set_draw_delta(5000 / opts.samples_per_pixel as u64);
    for (x, y, pixel) in image.enumerate_pixels_mut().progress_with(bar) {
        let mut colour = Colour::new(0.0, 0.0, 0.0);
        for _ in 0..opts.samples_per_pixel {
            // (u, v) measure from bottom left corner
            let u = (x as f64 + rand::random::<f64>()) / ((image_width - 1) as f64);
            let v = ((image_height - 1 - y) as f64 + rand::random::<f64>())
                / ((image_height - 1) as f64);
            let ray = camera.get_ray(u, v);
            colour += ray_colour(&ray, &world, opts.max_depth);
        }

        colour /= opts.samples_per_pixel as f64;
        // Gamma-correct for gamma=2
        colour = Colour {
            x: colour.x.sqrt(),
            y: colour.y.sqrt(),
            z: colour.z.sqrt(),
        };
        *pixel = colour.into();
    }

    println!("Saving image to '{}'", opts.output);
    image.save(opts.output)?;
    Ok(())
}

fn ray_colour<T: Surface>(ray: &RayR3, surface: &T, depth: u32) -> Colour {
    // Exceeded ray bounce limit; no more light is gathered
    if depth == 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    // 0.001 is to prevent collisions with the object the ray is leaving; the "acne" problem.
    if let Some(intersection) = surface.intersect(ray, 0.001, f64::INFINITY) {
        if let Some((colour, scatter)) = intersection.scatter(ray) {
            return colour.elementwise_mul(ray_colour(&scatter, surface, depth - 1));
        }
        return Colour::new(0.0, 0.0, 0.0);
    }

    // A simple gradient
    let unit_direction = ray.direction.as_unit();
    let t = (unit_direction.y + 1.0) / 2.0;
    Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
}

fn random_scene() -> Vec<Box<dyn Surface>> {
    let mut world: Vec<Box<dyn Surface>> = Vec::new();

    let ground_material = Lambertian::new(Colour::new(0.5, 0.5, 0.5));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f64 = rand::random();
            let center = Vec3::new(
                (a as f64) + 0.9 * rand::random::<f64>(),
                0.2,
                (b as f64) + 0.9 * rand::random::<f64>(),
            );
            let radius = 0.2;

            if (center - Vec3::new(4.0, radius, 0.0)).norm() < 0.9 {
                continue;
            }

            if choose_material < 0.8 {
                // Diffuse
                let colour = colour::random().elementwise_mul(colour::random());
                let material = Lambertian::new(colour);
                world.push(Box::new(Sphere::new(center, radius, material)));
            } else if choose_material < 0.95 {
                // Metal
                let colour = colour::random_range(0.5, 1.0);
                let fuzz = rand::random::<f64>() / 2.0;
                let material = Metal::new(colour, fuzz);
                world.push(Box::new(Sphere::new(center, radius, material)));
            } else {
                // Glass
                let material = Transparent::new(1.5);
                world.push(Box::new(Sphere::new(center, radius, material)));
            }
        }
    }

    let material = Transparent::new(1.5);
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Lambertian::new(Colour::new(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0);
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    world
}
