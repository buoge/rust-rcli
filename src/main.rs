// rcli csv -i input.csv -o output.json --header -d ','

// cargo run -- csv -i assets/juventus.csv
// cargo run -- csv -i assets/juventus.csv --format yaml

// cargo run  gen-pass -l 12

use clap::Parser;
use rust_rcli::{process_csv, process_genpass, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
    }

    Ok(())
}
