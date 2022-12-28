use std::collections::VecDeque;

pub struct TrendFollower {
    // The number of periods to use for moving average calculation
    periods: usize,
    // The current moving average
    moving_average: f64,
    // A queue to store the historical prices used for moving average calculation
    price_history: VecDeque<f64>,
}

impl TrendFollower {
    pub fn new(periods: usize) -> Self {
        Self {
            periods,
            moving_average: 0.0,
            price_history: VecDeque::new(),
        }
    }
    
    pub fn moving_average(&self) -> f64 {
        self.moving_average
    }

    pub fn update(&mut self, price: f64) {
        // Add the new price to the history
        self.price_history.push_back(price);
        // If we have more prices than the number of periods, remove the oldest price
        if self.price_history.len() > self.periods {
            self.price_history.pop_front();
        }
        // Recalculate the moving average
        let sum: f64 = self.price_history.iter().sum();
        self.moving_average = sum / self.price_history.len() as f64;
    }

    pub fn should_buy(&self, price: f64) -> bool {
        // If the current price is above the moving average, it may be a good time to buy
        price > self.moving_average
    }

    pub fn should_sell(&self, price: f64) -> bool {
        // If the current price is below the moving average, it may be a good time to sell
        price < self.moving_average
    }
}
