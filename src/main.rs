pub mod messages;
use anyhow::Result;
use clap::{Parser, Subcommand};
use gzp::{
    ZWriter,
    deflate::Mgzip,
    par::compress::{Compression, ParCompress, ParCompressBuilder},
};
use std::thread::available_parallelism;
use std::{
    fs::File,
    io::{Write, copy},
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Compress {
        #[command(subcommand)]
        opt: Compressopts,
    },
    Info,
}

#[derive(Subcommand, Clone)]
enum Compressopts {
    #[command(alias = "f")]
    File {
        source: String,
        destination: String,
    },
    Directory,
}

fn main() -> Result<()> {
    let total_threads = available_parallelism()?;
    let reserved_threads = total_threads.get() - 2;
    let cli = Cli::parse();
    match cli.cmd {
        None => {
            let dir = std::env::current_dir()?;
            let entries = std::fs::read_dir(dir)?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, std::io::Error>>()?;
            let mut count = 1;
            messages::none();
            for entry in entries {
                if entry.extension().and_then(|e| e.to_str()) == Some("mov") {
                    println!("{count}: {entry:?}");
                    count += 1;
                }
            }
        }
        Some(command) => match command {
            Commands::Compress { opt } => match opt {
                Compressopts::File {
                    source,
                    destination,
                } => {
                    let mut input = File::create(source)?;
                    let file_out = File::create(destination)?;
                    let mut compressor = ParCompressBuilder::<Mgzip>::new()
                        .num_threads(reserved_threads)?
                        .compression_level(Compression::fast())
                        .from_writer(file_out);

                    copy(&mut input, &mut compressor)?;
                    compressor.finish()?;

                    todo!()
                }
                Compressopts::Directory => {
                    todo!()
                }
            },
            Commands::Info => {
                todo!()
            }
        },
    }

    Ok(())
}
