mod client;
mod models;

// OK this is basically working.
// We need to define some controls or whatever in HA, query them, manipulate them.
//
// But more importantly we need to deep dive some concepts:

// - [x] std:error::Error trait - look it up, do we implement it?
// - [x] Debug vs Display

use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::Deserialize;

use crate::client::Client;
use crate::models::Error;

#[derive(Parser, Debug)]
#[command(name = "ha")]
#[command(name = "Home Assistant CLI")]
struct Cli {
    #[arg(long, env = "HA_HOST")]
    host: String,

    #[arg(long, env = "HA_API_KEY")]
    api_key: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    Components {},

    #[command()]
    States {},

    #[command()]
    On { state: String },

    #[command()]
    Off { state: String },
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let client = Client::new(args.host, args.api_key);

    match args.command {
        Commands::Components {} => {
            let response = client.get_components()?;
            for e in response {
                println!("COMPONENT: {}", e);
            }
        }
        Commands::States {} => {
            let response = client.get_states()?;
            for e in response {
                println!("State: {} {}", e.entity_id, e.state);
            }
        }
        Commands::On { state } => {
            client.set_state(&state, true)?;
            ()
        }
        Commands::Off { state } => {
            client.set_state(&state, false)?;
            ()
        }
    }

    Ok(())
}
