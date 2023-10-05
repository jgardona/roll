use clap::ValueEnum;
use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D20,
}

fn create_distribution(dtype: DiceType) -> rand::distributions::Uniform<usize> {
    match dtype {
        DiceType::D4 => rand::distributions::Uniform::new_inclusive(1, 4),
        DiceType::D6 => rand::distributions::Uniform::new_inclusive(1, 6),
        DiceType::D8 => rand::distributions::Uniform::new_inclusive(1, 8),
        DiceType::D10 => rand::distributions::Uniform::new_inclusive(1, 10),
        DiceType::D20 => rand::distributions::Uniform::new_inclusive(1, 20),
    }
}

fn roll_dice(distribution: rand::distributions::Uniform<usize>, amount: usize) -> Vec<usize> {
    let rng = rand::thread_rng();
    let results: Vec<usize> = rng.sample_iter(distribution).take(amount).collect();
    results
}

pub fn get_roll_results(dtype: DiceType, amount: usize) -> Vec<usize> {
    let distr = create_distribution(dtype);
    roll_dice(distr, amount)
}

#[cfg(test)]
mod tests {
    use super::{get_roll_results, DiceType};

    #[test]
    fn test_get_roll_results() {
        let results = get_roll_results(DiceType::D4, 4);
        dbg!(&results);
        assert_eq!(4, results.len());
    }
}
