use std::collections::VecDeque;

struct MovingAverage {
    window_size: usize,
    values: VecDeque<f64>,
    sum: f64,
}

impl MovingAverage {
    fn new(window_size: usize) -> MovingAverage {
        MovingAverage {
            window_size,
            values: VecDeque::new(),
            sum: 0.0,
        }
    }

    fn push(&mut self, value: f64) {
        self.sum += value;
        self.values.push_back(value);

        if self.values.len() > self.window_size {
            let popped = self.values.pop_front().unwrap();
            self.sum -= popped;
        }
    }

    fn average(&self) -> f64 {
        self.sum / self.values.len() as f64
    }
}

struct Trader {
    asset: String,
    balance: f64,
    fast_ma: MovingAverage,
    slow_ma: MovingAverage,
}

impl Trader {
    fn new(asset: String, balance: f64, fast_window: usize, slow_window: usize) -> Trader {
        Trader {
            asset,
            balance,
            fast_ma: MovingAverage::new(fast_window),
            slow_ma: MovingAverage::new(slow_window),
        }
    }

    fn update(&mut self, price: f64) {
        self.fast_ma.push(price);
        self.slow_ma.push(price);

        let fast_average = self.fast_ma.average();
        let slow_average = self.slow_ma.average();

        if fast_average > slow_average {
            // Buy asset
            self.balance -= price;
            println!("Bought {} at price {}", self.asset, price);
        } else if fast_average < slow_average {
            // Sell asset
            self.balance += price;
            println!("Sold {} at price {}", self.asset, price);
        }
    }
}

fn main() {
    let mut trader = Trader::new("BTC".to_string(), 1000.0, 10, 30);

    let prices = [7000.0, 7100.0, 7200.0, 7300.0, 7400.0, 7500.0];

    for price in prices.iter() {
        trader.update(*price);
    }
}
