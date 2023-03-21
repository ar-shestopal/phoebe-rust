#![allow(dead_code)]
#![allow(unused_variables)]

use crate::adapters::types::State;

#[derive(Debug, PartialEq)]
pub enum Signals {
    Buy,
    Sell,
    BuyNothing,
    SellNothing,
    StopBuy,
    StopSell,
    ByuSellSimultaneousSignals,
    StopBuyStopSellSimultaneousSignals,
    Nohting,
}

fn generate_signal(state: &State, avg_size: i32, avg_slope_range: i32) {
    if state.prices.len() < (avg_size - 1) as usize {
        println!("Not enough prices to calculate signal");
        return;
    }

    let signal = check_for_signal(state, avg_size, avg_slope_range);

    match signal {
        Signals::ByuSellSimultaneousSignals => {
            panic!("ERROR BUY AND SELL SIMULTANEOUS SIGNALS")
        }
        Signals::StopBuyStopSellSimultaneousSignals => {
            panic!("ERROR STOP BUY AND STOP SELL SIMULTANEOUS SIGNALS")
        }
        _ => {
            println!("Signal: {:?}", signal);
        }
    }
}

pub fn check_for_signal(state: &State, avg_size: i32, avg_slope_range: i32) -> Signals {
    let buy_sig = check_buy_conditions(state, avg_size, avg_slope_range);
    let sell_sig = check_sell_conditions(state, avg_size, avg_slope_range);

    match (buy_sig, sell_sig) {
        (Signals::Buy, Signals::SellNothing) => Signals::Buy,
        (Signals::BuyNothing, Signals::Sell) => Signals::Sell,
        (Signals::StopBuy, Signals::SellNothing) => Signals::StopBuy,
        (Signals::BuyNothing, Signals::StopSell) => Signals::StopSell,
        (Signals::Buy, Signals::Sell) => Signals::ByuSellSimultaneousSignals,
        (Signals::StopBuy, Signals::StopSell) => Signals::StopBuyStopSellSimultaneousSignals,
        _ => Signals::Nohting,
    }
}

fn check_buy_conditions(state: &State, avg_size: i32, avg_slope_range: i32) -> Signals {
    if state.prices.len() < (avg_size - 1) as usize {
        println!("Not enough prices to check buy conditions");
        return Signals::BuyNothing;
    }

    if state.avgs100.len() < avg_slope_range as usize {
        println!("Not enough avgs to check buy conditions");
        return Signals::BuyNothing;
    }

    let price = state.prices.last().unwrap();
    let avg = state.avgs100.last().unwrap();
    let slope = state.slopes.last().unwrap();
    let is_buy = state.is_buy;

    if is_buy {
        if price < avg && slope < &0.0 {
            Signals::StopBuy
        } else {
            Signals::BuyNothing
        }
    } else {
        if price > avg && slope > &0.0 {
            Signals::Buy
        } else {
            Signals::BuyNothing
        }
    }
}

fn check_sell_conditions(state: &State, avg_size: i32, avg_slope_range: i32) -> Signals {
    if state.prices.len() < (avg_size - 1) as usize {
        println!("Not enough prices to check sell conditions");
        return Signals::SellNothing;
    }

    if state.avgs100.len() < avg_slope_range as usize {
        println!("Not enough avgs to check sell conditions");
        return Signals::SellNothing;
    }

    let price = state.prices.last().unwrap();
    let avg = state.avgs100.last().unwrap();
    let slope = state.slopes.last().unwrap();
    let is_sell = state.is_sell;

    if is_sell {
        if price > avg && slope > &0.0 {
            Signals::StopSell
        } else {
            Signals::SellNothing
        }
    } else {
        if price < avg && slope < &0.0 {
            Signals::Sell
        } else {
            Signals::SellNothing
        }
    }
}
