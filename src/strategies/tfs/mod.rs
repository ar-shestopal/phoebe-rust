#![allow(dead_code)]
#![allow(unused_variables)]

mod signals;

extern crate serde;
use serde::{Deserialize, Serialize};

use crate::adapters::test_adapter::TestAdapter;
use crate::adapters::types::Adapter;
use crate::adapters::types::{CandleStick, CandleStickData};
use crate::strategies::strategy::Strategy;
use crate::strategies::state::State;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TfsState {
    pub is_buy: bool,
    pub is_sell: bool,
    pub profit: f32,
    pub prices: Vec<f32>,
    pub prev_candle: CandleStick,
    pub avgs100: Vec<f32>,
    pub sum100: f32,
    pub slopes: Vec<f32>,
    pub signals: Vec<String>,
    pub idx: i32,
}

impl TfsState {
    fn new() -> TfsState {
        TfsState {
            is_buy: false,
            is_sell: false,
            profit: 0.0,
            prices: Vec::new(),
            prev_candle: CandleStick {
                complete: true,
                volume: 0,
                time: "".to_string(),
                mid: CandleStickData {
                    o: 0.0,
                    h: 0.0,
                    l: 0.0,
                    c: 0.0,
                },
            },
            avgs100: Vec::new(),
            sum100: 0.0,
            slopes: Vec::new(),
            signals: Vec::new(),
            idx: 0,
        }
    }
}

impl State for TfsState {
    fn set_buy(&mut self) {
        self.is_buy = true;
        self.is_sell = false;
    }

    fn set_sell(&mut self) {
        self.is_buy = false;
        self.is_sell = true;
    }

    fn update_prices(&mut self) {
        let prices = self.prices.clone();
        let mut new_prices = Vec::new();
        for price in prices {
            new_prices.push(price);
        }
        self.prices = new_prices;
    }

    fn set_profit(&mut self, profit: f32) {
        self.profit = profit;
    }

    fn get_prices(&self) -> &Vec<f32> {
        &self.prices
    }
}

pub struct TfsStrategy {
    state: TfsState,
    adapter: Box<dyn Adapter>,
    iters_limit: i32,
    avg_size: i32,
    avg_sloper_range: i32,
}

impl TfsStrategy {
    pub fn new() -> Self {
        let ta = TestAdapter::new();
        Self {
            state: TfsState::new(),
            adapter: Box::new(ta),
            iters_limit: 100,
            avg_size: 100,
            avg_sloper_range: 15,
        }
    }
}


impl Strategy for  TfsStrategy {
    fn run_strategy(&mut self) {
        loop {
            self.run_iteration();
            if self.state.idx >= self.iters_limit {
                break;
            }
        }
    }

    fn run_iteration(&mut self) {
        #[cfg(test)]
        let data = self.adapter.get_data();
        self.state.idx += 1;
        #[cfg(not(test))]
        return
    }
}



