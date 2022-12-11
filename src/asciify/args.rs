use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLIArgs {
    /// Path to the file you wish to operate on
    pub path: String,

    #[command(subcommand)]
    pub mode: Mode
}

#[derive(Subcommand)]
pub enum Mode {
    /// Play a ASCII video file
    Play,

    /// Convert a video file into an ASCII video file
    Convert(ConvertArgs)
}

#[derive(Args)]
pub struct ConvertArgs {
    /// The framerate of the video
    pub framerate: u32,

    /// Downscale multiplayer (recommended: 5..10). Larger number means smaller output resoltuion.
    pub step: u32,

    /// Name of the ASCII video file
    pub save_path: String,

    /// Number of frames to load
    pub frame_count: Option<usize>
}

#[derive(Args)]
pub struct ConvertAndPlayArgs {
    pub framerate: u32,
    pub step: u32,
    pub frame_count: Option<usize>
}