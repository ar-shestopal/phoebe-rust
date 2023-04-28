#![allow(dead_code)]
#![allow(unused_variables)]

use crate::strategies::state::State;

use super::types::{ PricesResponse, CandleStick, CandleStickData, Adapter };
use std::error::Error;
use std::cell::Cell;


pub struct TestAdapter {
    index: Cell<usize>,
    prices: Vec<Vec<PricesResponse>>,
}

impl TestAdapter {
    pub fn new() -> TestAdapter {
        let prices = read_form_csv().unwrap();
        let window = prices.windows(100);
        let mut prices_vec = Vec::new();

        for w in window {
            prices_vec.push(w.to_vec());
        }

        TestAdapter { index: Cell::new(0), prices: prices_vec }
    }
}

impl Adapter for TestAdapter {
    fn get_data(&self) -> Vec<PricesResponse> {
        let i = self.index.get();
        self.index.set(i+1);
        self.prices[i].clone()
    }

    fn buy_request(&self, state: Box<dyn State>) -> String {
        "buy request".to_string()
    }

    fn sell_request(&self, state: Box<dyn State>) -> String {
        "sell request".to_string()
    }

    fn stop_buy_request(&self, state: Box<dyn State>) -> String {
        "stop buy request".to_string()
    }

    fn stop_sell_request(&self, state: Box<dyn State>) -> String {
        "stop sell request".to_string()
    }

    fn update_profit(&mut self, state: Box<dyn State>) -> String {
        // state.set_profit(0.0);
        "Profit updated".to_string()
    }
}

#[derive(Debug, serde::Deserialize)]
struct Row {
    #[serde(rename = "Time")]
    pub time: String,
    #[serde(rename = "Open")]
    pub open: String,
    #[serde(rename = "High")]
    pub high: String,
    #[serde(rename = "Low")]
    pub low: String,
    #[serde(rename = "Close")]
    pub close: String,
    #[serde(rename = "Volume")]
    pub volume: String,
}

fn read_form_csv() -> Result<Vec<PricesResponse>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_path("data/GBPNZD_M15.csv")?;
    let mut prices = Vec::new();
    for result in rdr.deserialize() {
        let record: Row = result?;

        let candlestick_data = CandleStickData {
            o: record.open.parse::<f32>()?,
            h: record.high.parse::<f32>()?,
            l: record.low.parse::<f32>()?,
            c: record.close.parse::<f32>()?,
        };

        let candlestick = CandleStick {
            complete: true,
            volume: 0,
            time: record.time,
            mid: candlestick_data,
        };

        let price_response = PricesResponse {
            instruments: "GBPNZD".to_string(),
            granularity: "M15".to_string(),
            candles: vec![candlestick],
        };
        prices.push(price_response);
    }
    Ok(prices)
}
