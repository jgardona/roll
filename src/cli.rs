use clap::Parser;

use crate::dice::{get_toss_results, DiceType};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The type of dice to toss
    #[arg(value_enum)]
    dices: DiceType,

    /// The amount of dices to toss
    amount: usize,
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();
    println!("\nProcessing your results\n");
    let results = get_toss_results(cli.dices, cli.amount);

    for (i, e) in results.iter().enumerate() {
        let index = i + 1;
        print!("⚄  {index},{e} ");
    }

    let total: usize = results.iter().sum();
    println!("\n\nTotal of tosses: {total}");

    Ok(())
}

#[cfg(test)]
mod cli_test {
    #[test]
    fn itworks() {}
}
