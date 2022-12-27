use std::collections::HashMap;

struct Arbitrageur {
    // A map of prices for each asset
    prices: HashMap<String, f64>,
    // The current holding of each asset
    holdings: HashMap<String, f64>,
}

impl Arbitrageur {
    fn new() -> Self {
        Self {
            prices: HashMap::new(),
            holdings: HashMap::new(),
        }
    }

    fn update_price(&mut self, asset: String, price: f64) {
        self.prices.insert(asset, price);
    }

    fn update_holding(&mut self, asset: String, holding: f64) {
        self.holdings.insert(asset, holding);
    }

    fn get_profit(&self, asset1: String, asset2: String) -> f64 {
        // Calculate the profit from buying one asset and selling the other
        let price1 = self.prices.get(&asset1).unwrap_or(&0.0);
        let price2 = self.prices.get(&asset2).unwrap_or(&0.0);
        let holding1 = self.holdings.get(&asset1).unwrap_or(&0.0);
        let holding2 = self.holdings.get(&asset2).unwrap_or(&0.0);
        (price2 - price1) * holding1 + (price1 - price2) * holding2
    }

    fn execute_arbitrage(&mut self, asset1: String, asset2: String) {
        // Calculate the optimal trade size to maximize profit
        let price1 = self.prices.get(&asset1).unwrap_or(&0.0);
        let price2 = self.prices.get(&asset2).unwrap_or(&0.0);
        let holding1 = self.holdings.get(&asset1).unwrap_or(&0.0);
        let holding2 = self.holdings.get(&asset2).unwrap_or(&0.0);
        let trade_size = (*holding2 - *holding1 * *price2 / *price1).abs();
        // Execute the trade
        if *price1 > *price2 {
            self.update_holding(asset1, *holding1 - trade_size);
            self.update_holding(asset2, *holding2 + trade_size * *price2 / *price1);
        } else {
            self.update_holding(asset1, *holding1 + trade_size * *price1 / *price2);
            self.update_holding(asset2, *holding2 - trade_size);
        }
    }
}
