mod args;
mod chunk;
mod chunk_type;
// mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use args::{Commands, CLI};
use clap::Parser;

fn main() -> Result<()> {
    let cli = CLI::parse();

    match &cli.command {
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

        _ => println!("There is no such command"),
    }

    Ok(())
}
