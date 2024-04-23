// rcli csv -i input.csv -o output.json --header -d ','

// cargo run -- csv -i assets/juventus.csv

use clap::Parser;
use rust_rcli::{Opts,SubCommand,process_csv};

fn main() ->anyhow::Result<()>{
    println!("Hello, world!");

    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }

    Ok(())
}
