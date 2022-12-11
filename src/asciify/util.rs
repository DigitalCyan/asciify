use std::{
    error::Error,
    fs::{File, self},
    io::{BufReader, BufWriter},
    path::Path,
    process::Command,
};

use image::DynamicImage;
use image::GenericImageView;
use serde_cbor::{from_reader, to_writer};

use super::types::ASCIIVideo;

pub fn save_video(path: &Path, video: &ASCIIVideo) -> Result<(), Box<dyn Error>> {
    let writer = BufWriter::new(File::create(path)?);

    to_writer(writer, video)?;

    Ok(())
}

pub fn load_video(path: &Path) -> Result<ASCIIVideo, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);

    Ok(from_reader(reader).unwrap())
}

pub fn get_char_for_pixel(image: &DynamicImage, x: u32, y: u32, step: u32) -> char {
    let color = image.get_pixel(x * step, y * step);

    let c = match color.0[1] {
        0..=5 => ' ',

        6..=20 => '\'',

        3..=30 => '"',

        31..=50 => '*',

        51..=200 => '%',

        _ => '#',
    };

    c
}

pub fn video_to_frames(path: &Path, framerate: u32) {
    let _ = fs::create_dir("/tmp/frames/");

    Command::new("ffmpeg")
        .arg("-i")
        .arg(format!("{}", path.as_os_str().to_str().unwrap()).as_str())
        .arg("-r")
        .arg(format!("{}", framerate).as_str())
        .arg("/tmp/frames/%d.bmp")
        .output().unwrap_or_else(|_| {
            println!("Failed to run ffmpeg. Is it installed?");
            std::process::exit(1);
        });
}
