use std::collections::VecDeque;

struct MeanReversionTrader {
    // The number of periods to use for calculating the mean and standard deviation
    periods: usize,
    // The current mean of the price history
    mean: f64,
    // The current standard deviation of the price history
    stddev: f64,
    // A queue to store the historical prices used for mean and standard deviation calculation
    price_history: VecDeque<f64>,
}

impl MeanReversionTrader {
    fn new(periods: usize) -> Self {
        Self {
            periods,
            mean: 0.0,
            stddev: 0.0,
            price_history: VecDeque::new(),
        }
    }

    fn update(&mut self, price: f64) {
        // Add the new price to the history
        self.price_history.push_back(price);
        // If we have more prices than the number of periods, remove the oldest price
        if self.price_history.len() > self.periods {
            self.price_history.pop_front();
        }
        // Recalculate the mean and standard deviation
        let sum: f64 = self.price_history.iter().sum();
        self.mean = sum / self.price_history.len() as f64;
        let variance: f64 = self.price_history
            .iter()
            .map(|x| (x - self.mean).powi(2))
            .sum::<f64>()
            / self.price_history.len() as f64;
        self.stddev = variance.sqrt();
    }

    fn should_buy(&self, price: f64) -> bool {
        // If the current price is below the mean minus a certain number of standard deviations, it may be a good time to buy
        price < self.mean - 2.0 * self.stddev
    }

    fn should_sell(&self, price: f64) -> bool {
        // If the current price is above the mean plus a certain number of standard deviations, it may be a good time to sell
        price > self.mean + 2.0 * self.stddev
    }
}
