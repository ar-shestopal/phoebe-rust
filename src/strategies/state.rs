pub trait State {
    fn set_buy(&mut self);
    fn set_sell(&mut self);
    fn update_prices(&mut self);
    fn set_profit(&mut self, profit: f32);
    fn get_prices(&self) -> &Vec<f32>;
}
