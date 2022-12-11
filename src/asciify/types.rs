use std::path::Path;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ASCIIVideo {
    pub framerate: u32,
    pub resolution: (u32, u32),
    pub frames: Vec<String>
}

impl ASCIIVideo {
    pub fn new(framerate: u32, resolution: (u32, u32)) -> Self {
        Self {
            framerate,
            resolution,
            frames: Vec::new()
        }
    }
}

pub struct LoadSettings<'a> {
    pub path: &'a Path,
    pub step: u32,
    pub framerate: u32
}