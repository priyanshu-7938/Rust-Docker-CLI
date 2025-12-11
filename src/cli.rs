use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Docker cli tool")]
#[command(about = "It is a cli tool ot controle docker containers, and more.")]
pub struct Cli{
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command{
    List{
        #[command(subcommand)]
        list_command: ListCommands,
    },
    Start{
        container_name: String,
    },
    Stop{
        container_name: String,
    },
    Pull{
        image_name: String,
    }
}

#[derive(Subcommand)]
pub enum ListCommands{
    Containers {
        #[arg(short, long)]
        all: bool
    },

    Images {
        #[arg(short, long)]
        all: bool
    }
}