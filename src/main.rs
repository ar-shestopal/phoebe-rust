mod adapters;
mod strategies;
fn main() {
    strategies::tfs::run_strategy(100, 100, 100);
}
