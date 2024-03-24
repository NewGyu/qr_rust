use clap::Parser;

fn main() {
    let _args = args::Args::parse();
    println!("Hello, world!");
}

mod args {
    use super::Parser;

    #[derive(Debug, Parser)]
    #[command(about = "A QR code decoder.", version, author)]
    pub struct Args {
        file: String,
    }
}
