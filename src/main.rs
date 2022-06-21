use clap::Parser;

use crate::args::Args;
use crate::features::extract::extract as feature_extract;

mod args;
mod common;
mod features;
mod sensor_msgs;

fn main() {
    let args = Args::parse();
    feature_extract::extract(args).unwrap();
}
