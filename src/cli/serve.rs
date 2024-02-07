use clap::Parser;

use crate::{explorer, rest};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Mode {
    Explorer,
    Rest,
}

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
pub struct Args {
    /// Run program in certain mode
    #[arg(short, long)]
    mode: Mode,

    #[arg(short, long, default_value_t = 3000)]
    port: usize,
}

pub async fn serve() {
    let args = Args::parse();

    match args.mode {
        Mode::Explorer => explorer::serve::serve(args.port).await,
        Mode::Rest => rest::serve::serve(args.port).await,
    }
}
