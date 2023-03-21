extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
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

impl State {
    pub fn new() -> State {
        State {
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

pub trait Adapter {
    fn update_profit(&mut self, state: &mut State);
    fn buy_request(&self, state: &mut State) -> String;
    fn sell_request(&self, state: &mut State) -> String;
    fn stop_buy_request(&self, state: &mut State) -> String;
    fn stop_sell_request(&self, state: &mut State) -> String;
    fn get_data(&self) -> PricesResponse;
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