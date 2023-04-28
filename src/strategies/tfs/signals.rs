#![allow(dead_code)]
#![allow(unused_variables)]

use crate::strategies::tfs::TfsState;

use crate::strategies::signals::BuySellConditionsChecker;

// fn check_buy_conditions(state: &TfsState, avg_size: i32, avg_slope_range: i32) -> Signals {
//     if state.prices.len() < (avg_size - 1) as usize {
//         println!("Not enough prices to check buy conditions");
//         return Signals::BuyNothing;
//     }
//
//     if state.avgs100.len() < avg_slope_range as usize {
//         println!("Not enough avgs to check buy conditions");
//         return Signals::BuyNothing;
//     }
//
//     let price = state.prices.last().unwrap();
//     let avg = state.avgs100.last().unwrap();
//     let slope = state.slopes.last().unwrap();
//     let is_buy = state.is_buy;
//
//     if is_buy {
//         if price < avg && slope < &0.0 {
//             Signals::StopBuy
//         } else {
//             Signals::BuyNothing
//         }
//     } else {
//         if price > avg && slope > &0.0 {
//             Signals::Buy
//         } else {
//             Signals::BuyNothing
//         }
//     }
// }
//
// fn check_sell_conditions(state: &TfsState, avg_size: i32, avg_slope_range: i32) -> Signals {
//     if state.prices.len() < (avg_size - 1) as usize {
//         println!("Not enough prices to check sell conditions");
//         return Signals::SellNothing;
//     }
//
//     if state.avgs100.len() < avg_slope_range as usize {
//         println!("Not enough avgs to check sell conditions");
//         return Signals::SellNothing;
//     }
//
//     let price = state.prices.last().unwrap();
//     let avg = state.avgs100.last().unwrap();
//     let slope = state.slopes.last().unwrap();
//     let is_sell = state.is_sell;
//
//     if is_sell {
//         if price > avg && slope > &0.0 {
//             Signals::StopSell
//         } else {
//             Signals::SellNothing
//         }
//     } else {
//         if price < avg && slope < &0.0 {
//             Signals::Sell
//         } else {
//             Signals::SellNothing
//         }
//     }
// }

