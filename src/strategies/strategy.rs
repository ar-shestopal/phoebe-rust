pub trait Strategy {
    fn run_strategy(&mut self);
    fn run_iteration(&mut self);
}
