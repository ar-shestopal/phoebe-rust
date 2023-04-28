mod adapters;
mod strategies;

use strategies::{tfs::TfsStrategy, strategy::Strategy};

fn main() {
    let mut strategy = TfsStrategy::new();
    strategy.run_strategy();
}
