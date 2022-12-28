mod trend_follower;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: trend_follower <periods> <price1> <price2> ... <priceN>");
        std::process::exit(1);
    }

    let periods = args[1].parse::<usize>().unwrap();
    let prices: Vec<f64> = args[2..].iter().map(|s| s.parse::<f64>().unwrap()).collect();

    let mut trend_follower = trend_follower::TrendFollower::new(periods);

    for price in prices {
        trend_follower.update(price);
        println!("Current price: {} Moving average: {}", price, trend_follower.moving_average());
    }
}
