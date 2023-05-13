use std::{
    path::Path, process::exit,
};
//import fat32 module
mod file_types;
mod cli;
mod utils;
mod format;
mod recover;

use crate::{utils::create_dir_or_default};
use cli::CliArgs;
use clap::Parser;
use format::start_format;
use recover::start_recover;

fn main() {
    let args: CliArgs = CliArgs::parse();
    
    // check if device exists
    let device = args.device;
    if !Path::new(&device).exists() {
        println!("Device {} does not exist", &device);
        exit(1);
    }

    match args.subcmd {
        cli::Commands::Recover { output, file_type } => {
            let output_path = create_dir_or_default(output);
            start_recover(device, file_type, output_path);
        }

        cli::Commands::Format => {
            start_format(device);
        }
    }
}