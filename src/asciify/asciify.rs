use std::{error::Error, fs, io::Write, path::Path, time::SystemTime};

use image::open;

use crate::asciify::util::get_char_for_pixel;

use super::{
    args::{CLIArgs, Mode},
    types::{ASCIIVideo, LoadSettings},
    util::{load_video, save_video, video_to_frames},
};

pub fn process_args(args: CLIArgs) {
    let path = Path::new(&args.path);

    if !path.is_file() {
        println!("The provided path should lead to a file");
        std::process::exit(1);
    }

    match args.mode {
        Mode::Play => {
            let video = load_video(path).unwrap_or_else(|_| {
                println!("Failed to open file.");
                std::process::exit(1);
            });
            play(&video);
        }

        Mode::Convert(cargs) => {
            let settings = LoadSettings {
                framerate: cargs.framerate,
                step: cargs.step,
                path: Path::new("/tmp/frames/"),
            };

            video_to_frames(path, settings.framerate);

            let video = match cargs.frame_count {
                Some(n) => load_n_images(&settings, n),
                None => load_image_sequence(&settings),
            }
            .unwrap_or_else(|_| {
                println!("Failed to load frames.");
                std::process::exit(1);
            });

            let path = Path::new(&cargs.save_path);

            save_video(path, &video).unwrap_or_else(|_| {
                println!("Failed to save ASCII video.");
                std::process::exit(1);
            });
        }
    }
}

pub fn load_image_sequence(settings: &LoadSettings) -> Result<ASCIIVideo, Box<dyn Error>> {
    let LoadSettings {
        framerate,
        step,
        path,
    } = settings;

    let frame_count = fs::read_dir(path)?.count();

    let image = open(path.join("1.bmp"))?;

    let input_resolution = (image.width(), image.height());
    let output_resolution = (input_resolution.0 / step, input_resolution.1 / step);

    let mut video = ASCIIVideo::new(framerate.clone(), output_resolution);

    for i in 1..frame_count {
        let file_name = format!("{}.bmp", i);

        println!("Loading {}", file_name);

        let image = open(path.join(file_name))?;

        let mut frame = String::new();
        for y in 0..output_resolution.1 - 1 {
            for x in 0..output_resolution.0 - 1 {
                let c = get_char_for_pixel(&image, x, y, settings.step);

                frame.push(c);
                frame.push(' ');
            }

            frame.push('\n');
        }

        video.frames.push(frame);
    }

    Ok(video)
}

pub fn load_n_images(settings: &LoadSettings, n: usize) -> Result<ASCIIVideo, Box<dyn Error>> {
    let LoadSettings {
        framerate,
        step,
        path,
    } = settings;

    let image = open(path.join("1.bmp"))?;

    let input_resolution = (image.width(), image.height());
    let output_resolution = (input_resolution.0 / step, input_resolution.1 / step);

    let mut video = ASCIIVideo::new(framerate.clone(), output_resolution);

    for i in 1..n {
        let file_name = format!("{}.bmp", i);

        println!("Loading {}", file_name);

        let image = open(path.join(file_name))?.grayscale();

        let mut frame = String::new();
        for y in 0..output_resolution.1 - 1 {
            for x in 0..output_resolution.0 - 1 {
                let c = get_char_for_pixel(&image, x, y, settings.step);

                frame.push(c);
                frame.push(' ');
            }

            frame.push('\n');
        }

        video.frames.push(frame);
    }

    Ok(video)
}

pub fn play(video: &ASCIIVideo) {
    let frame_delay = 1.0 / video.framerate as f64;
    let mut stdout = std::io::stdout().lock();

    for frame in video.frames.iter() {
        let now = SystemTime::now();
        stdout.write(frame.as_bytes()).unwrap();

        while now.elapsed().unwrap().as_secs_f64() <= frame_delay {}
    }
}
