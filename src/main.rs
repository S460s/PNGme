mod args;
mod chunk;
mod chunk_type;
// mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use std::fs::{self, File};
use std::io::{Read, Write};
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
            let chunk_type = ChunkType::from_str(chunk_type)?;
            let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

            let mut file = File::options().append(true).open(file_path)?;
            let res = file.write(chunk.as_bytes().as_ref())?;

            println!("{res} bytes written to {file_path:?} successfully");
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

            println!("Message -> {}", chunk.data_as_string()?);
        }

        Commands::Remove {
            file_path,
            chunk_type,
        } => {
            todo!()
        }

        Commands::Print { file_path } => {
            todo!()
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
