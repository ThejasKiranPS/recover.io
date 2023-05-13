use clap::{Parser, Subcommand};

use crate::file_types::FileTypeInfo;

#[derive(Parser, Debug)]
#[clap(name = "recover.io", version = "0.0.1")]
pub struct CliArgs {
    /// The path to the device to recover files from
    #[clap(short, long)]
    pub device: String,

    #[clap(subcommand)]
    pub subcmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Recover files
    Recover {
        /// The path to the directory to recover files to
        #[clap(short, long)]
        output: Option<String>,
        /// The file type to recover
        #[clap(short, long)]
        file_type: FileTypeInfo,
    },
    /// Format Securely
    Format,
}
