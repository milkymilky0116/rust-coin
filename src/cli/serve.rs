use clap::Parser;
use tokio::try_join;

use crate::{explorer, rest};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Mode {
    Html,
    Rest,
    Both,
}

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
pub struct Args {
    /// Run program in certain mode
    #[arg(short, long, value_enum,default_value_t = Mode::Rest)]
    mode: Mode,

    #[arg(short, long, default_value_t = 3000)]
    port: usize,
}

pub async fn serve() {
    let args = Args::parse();

    match args.mode {
        Mode::Html => explorer::serve::serve(args.port).await,
        Mode::Rest => rest::serve::serve(args.port).await,
        Mode::Both => match try_join!(
            tokio::spawn(explorer::serve::serve(args.port)),
            tokio::spawn(rest::serve::serve(args.port + 1000))
        )
        .unwrap()
        {
            _ => {}
        },
    }
}
