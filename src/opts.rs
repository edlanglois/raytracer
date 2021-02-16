use crate::ratio::Ratio;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Eric Langlois")]
pub struct Opts {
    #[clap(short, long, default_value = "images/image.png")]
    pub output: String,

    #[clap(short, long, default_value = "16:9")]
    pub aspect_ratio: Ratio<u32>,

    #[clap(long, default_value = "400")]
    pub width: u32,

    #[clap(short, long, default_value = "100")]
    pub samples_per_pixel: u32,

    #[clap(long, default_value = "50")]
    pub max_depth: u32,
}
