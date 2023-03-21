#![allow(dead_code)]
#![allow(unused_variables)]

use crate::adapters::test_adapter::TestAdapter;
use crate::adapters::types::State;

pub fn run_strategy(iters_limit: i32, avg_size: i32, avg_sloper_range: i32) {
    let adaper = TestAdapter::new();
    let mut state = State::new();

    loop {
        run_iteration(&mut state, &adaper, avg_size, avg_sloper_range);
        if state.idx >= iters_limit {
            break;
        }
    }
}

pub fn run_iteration(state: &mut State, adapter: &TestAdapter, avg_size: i32, avg_sloper_range: i32) {
    #[cfg(test)]
    let data = adapter.get_data();
    state.idx += 1;
    #[cfg(not(test))]
    return
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_iteration() {

        let mut state = State::new();
        let adapter = TestAdapter::new();
        run_iteration(&mut state, &adapter, 1, 1);

        assert_eq!(state.idx, 1);
    }
}
