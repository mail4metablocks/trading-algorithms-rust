use std::collections::HashMap;

struct PortfolioOptimizer {
    // A map of expected returns for each asset
    expected_returns: HashMap<String, f64>,
    // A map of variances for each asset
    variances: HashMap<String, f64>,
    // The current holding of each asset
    holdings: HashMap<String, f64>,
    // The risk tolerance of the portfolio
    risk_tolerance: f64,
}

impl PortfolioOptimizer {
    fn new(risk_tolerance: f64) -> Self {
        Self {
            expected_returns: HashMap::new(),
            variances: HashMap::new(),
            holdings: HashMap::new(),
            risk_tolerance,
        }
    }

    fn update_expected_return(&mut self, asset: String, expected_return: f64) {
        self.expected_returns.insert(asset, expected_return);
    }

    fn update_variance(&mut self, asset: String, variance: f64) {
        self.variances.insert(asset, variance);
    }

    fn update_holding(&mut self, asset: String, holding: f64) {
        self.holdings.insert(asset, holding);
    }

    fn optimize(&mut self) {
        // Calculate the portfolio's expected return and variance
        let mut portfolio_expected_return = 0.0;
        let mut portfolio_variance = 0.0;
        for (asset, holding) in &self.holdings {
            let expected_return = self.expected_returns.get(asset).unwrap_or(&0.0);
            let variance = self.variances.get(asset).unwrap_or(&0.0);
            portfolio_expected_return += *expected_return * *holding;
            portfolio_variance += *variance * *holding * *holding;
        }
        // Rebalance the portfolio to meet the desired risk tolerance
        let target_variance = self.risk_tolerance * self.risk_tolerance * portfolio_expected_return;
        let variance_difference = target_variance - portfolio_variance;
        for (asset, holding) in &self.holdings {
            let expected_return = self.expected_returns.get(asset).unwrap_or(&0.0);
            let variance = self.variances.get(asset).unwrap_or(&0.0);
            let new_holding = *holding + variance_difference * *expected_return / (*variance + portfolio_variance);
            self.update_holding(asset.to_owned(), new_holding);
        }
    }
}
