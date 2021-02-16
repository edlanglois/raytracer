use anyhow;
use clap::Clap;
use image::RgbImage;
use indicatif::{ProgressBar, ProgressIterator};
use rand;
use raytracer::materials::{Lambertian, Metal, Transparent};
use raytracer::objects::Sphere;
use raytracer::Opts;
use raytracer::{Camera, Colour, RayR3, Surface, Vec3};
use std::f64;

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

fn main() -> Result<(), anyhow::Error> {
    let opts: Opts = Opts::parse();
    let image_width = opts.width;
    let image_height = opts.aspect_ratio.a_to_b(image_width);
    println!("Image dimensions: {} x {}", image_width, image_height);

    // Image
    let mut image = RgbImage::new(image_width, image_height);

    // World
    let mut world: Vec<Box<dyn Surface>> = Vec::new();
    // Ground
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian {
            colour: Colour::new(0.8, 0.8, 0.0),
        },
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        // Lambertian {
        //     colour: Colour::new(0.7, 0.3, 0.3),
        // },
        // Transparent::new(1.5),
        Lambertian {
            colour: Colour::new(0.1, 0.2, 0.5),
        },
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        // Metal::new(Colour::new(0.8, 0.8, 0.8), 0.3),
        Transparent::new(1.5),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Metal::new(Colour::new(0.8, 0.6, 0.2), 0.0),
    )));
    // world.push(Box::new(Sphere::new(
    //     Vec3::new(0.5, 0.0, -0.7),
    //     0.2,
    //     Transparent::new(1.5),
    // )));
    // world.push(Box::new(Sphere::new(
    //     Vec3::new(0.0, 0.3, -5.0),
    //     0.4,
    //     Lambertian {
    //         colour: Colour::new(0.7, 0.3, 0.3),
    //     },
    // )));

    // Camera
    let camera = Camera::new(90.0, opts.aspect_ratio.into());

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
