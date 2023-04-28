#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;

use super::state::State;

pub trait BuySellConditionsChecker {
    fn check_buy_conditions(&self, state: Rc<dyn State>, avg_size: i32, avg_slope_range: i32) -> Signals;
    fn check_sell_conditions(&self, state: Rc<dyn State>, avg_size: i32, avg_slope_range: i32) -> Signals;
}

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

fn generate_signal(checker: Box<dyn BuySellConditionsChecker>, state: Rc<dyn State>, avg_size: i32, avg_slope_range: i32) {
    if state.get_prices().len() < (avg_size - 1) as usize {
        println!("Not enough prices to calculate signal");
        return;
    }

    let signal = check_for_signal(checker, state, avg_size, avg_slope_range);

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

pub fn check_for_signal(checker: Box<dyn BuySellConditionsChecker>, state: Rc<dyn State>, avg_size: i32, avg_slope_range: i32) -> Signals {
    let buy_sig = checker.check_buy_conditions(state.clone(), avg_size, avg_slope_range);
    let sell_sig = checker.check_sell_conditions(state.clone(), avg_size, avg_slope_range);

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

