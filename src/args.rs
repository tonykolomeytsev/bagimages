use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about=None)]
pub struct Args {
    /// Path to the bag file.
    pub path_to_bag: String,
    /// Path to output directory.
    pub output_dir: String,
    /// The name of the topics from which you want to export images.
    pub topics: Vec<String>,
    /// Time (in seconds) from which to start exporting.
    #[clap(short, long, default_value_t = 0f64)]
    pub start: f64,
    /// Time (in seconds) until which export should continue
    /// [optional]
    #[clap(short, long)]
    pub end: Option<f64>,
    /// Number of frames to be exported.
    /// If it's not specified, all frames will be exported
    /// [optional]
    #[clap(short, long)]
    pub number: Option<u32>,
    /// Step by which frames should be exported.
    #[clap(short = 'S', long, default_value_t = 1u32)]
    pub step: u32,
    /// Convert RGB8 to BGR8 (for case cv_bridge mixed up color channels)
    #[clap(short, default_value_t = true)]
    pub invert_channels: bool,
}
