extern crate serde;
use serde::{Deserialize, Serialize};

use crate::strategies::state::State;

pub trait Adapter {
    fn update_profit(&mut self, state: Box<dyn State>) -> String;
    fn sell_request(&self, state: Box<dyn State>) -> String;
    fn buy_request(&self, state: Box<dyn State>) -> String;
    fn stop_buy_request(&self, state: Box<dyn State>) -> String;
    fn stop_sell_request(&self, state: Box<dyn State>) -> String;
    fn get_data(&self) -> Vec<PricesResponse>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PricesResponse {
    pub instruments: String,
    pub granularity: String,
    pub candles: Vec<CandleStick>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandleStickData {
    pub o: f32,
    pub h: f32,
    pub l: f32,
    pub c: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandleStick {
    pub complete: bool,
    pub volume: i32,
    pub time: String,
    pub mid: CandleStickData,
}
