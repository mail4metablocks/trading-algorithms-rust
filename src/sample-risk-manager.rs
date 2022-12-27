use std::collections::HashMap;

struct RiskManager {
    // A map of maximum allowed positions for each asset
    max_positions: HashMap<String, f64>,
    // The current holding of each asset
    holdings: HashMap<String, f64>,
    // The current market value of each asset
    market_values: HashMap<String, f64>,
    // The maximum allowed market value for the portfolio
    max_market_value: f64,
}

impl RiskManager {
    fn new(max_market_value: f64) -> Self {
        Self {
            max_positions: HashMap::new(),
            holdings: HashMap::new(),
            market_values: HashMap::new(),
            max_market_value,
        }
    }

    fn set_max_position(&mut self, asset: String, max_position: f64) {
        self.max_positions.insert(asset, max_position);
    }

    fn update_holding(&mut self, asset: String, holding: f64) {
        self.holdings.insert(asset, holding);
    }

    fn update_market_value(&mut self, asset: String, market_value: f64) {
        self.market_values.insert(asset, market_value);
    }

    fn enforce_limits(&mut self) {
        // Calculate the current market value of the portfolio
        let mut portfolio_market_value = 0.0;
        for (asset, holding) in &self.holdings {
            let market_value = self.market_values.get(asset).unwrap_or(&0.0);
            portfolio_market_value += *market_value * *holding;
        }
        // Check if the portfolio market value exceeds the maximum allowed
        if portfolio_market_value > self.max_market_value {
            // Calculate the excess market value
            let excess_market_value = portfolio_market_value - self.max_market_value;
            // Distribute the excess market value among the positions to reduce them to their maximum allowed levels
            for (asset, holding) in &self.holdings {
                let max_position = self.max_positions.get(asset).unwrap_or(&0.0);
                let market_value = self.market_values.get(asset).unwrap_or(&0.0);
                let excess_market_value_for_asset = excess_market_value * *holding / *market_value;
                let new_holding = *max_position - excess_market_value_for_asset;
                self.update_holding(asset.to_owned(), new_holding);
            }
        }
    }
}
