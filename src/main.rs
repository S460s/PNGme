mod args;
mod chunk;
mod chunk_type;
// mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use std::fs::{self, File};
use std::io::Write;
use std::str::FromStr;

use args::{Commands, CLI};
use clap::Parser;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

fn main() -> Result<()> {
    let cli = CLI::parse();

    match &cli.command {
        Commands::Encode {
            message,
            file_path,
            chunk_type,
        } => {
            let file = fs::read(file_path)?;
            let mut png = Png::try_from(file.as_slice())?;
            let chunk_type = ChunkType::from_str(chunk_type)?;
            let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
            png.append_chunk(chunk);

            let mut file = File::options().write(true).truncate(true).open(file_path)?;
            file.write(png.as_bytes().as_ref())?;
        }

        Commands::Decode {
            file_path,
            chunk_type,
        } => {
            let file = fs::read(file_path)?;
            let png = Png::try_from(file.as_slice())?;

            let chunk = png
                .chunk_by_type(chunk_type)
                .ok_or("no such chunk in the specified file")?;

            println!("{}", chunk.data_as_string()?);
        }

        Commands::Remove {
            file_path,
            chunk_type,
        } => {
            // look into file reading and writing
            let file = fs::read(file_path)?;
            let mut png = Png::try_from(file.as_slice())?;

            if let Ok(chunk) = png.remove_chunk(chunk_type) {
                println!("Removed chunk:\n\n {chunk}");
                let mut file = File::options().write(true).truncate(true).open(file_path)?;
                file.write(png.as_bytes().as_ref())?;
            } else {
                println!("no such chunk in the specified file")
            };
        }

        Commands::Print { file_path } => {
            let file = fs::read(file_path)?;
            let png = Png::try_from(file.as_slice())?;

            println!("{png}")
        }
        Commands::Banner => {
            println!(
                "\n\n:::::::::  ::::    :::  ::::::::  ::::    ::::  :::::::::: 
:+:    :+: :+:+:   :+: :+:    :+: +:+:+: :+:+:+ :+:        
+:+    +:+ :+:+:+  +:+ +:+        +:+ +:+:+ +:+ +:+        
+#++:++#+  +#+ +:+ +#+ :#:        +#+  +:+  +#+ +#++:++#   
+#+        +#+  +#+#+# +#+   +#+# +#+       +#+ +#+        
#+#        #+#   #+#+# #+#    #+# #+#       #+# #+#        
###        ###    ####  ########  ###       ### ########## \n\n"
            )
        }
    }

    Ok(())
}
