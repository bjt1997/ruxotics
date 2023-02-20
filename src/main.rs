use std::time::Instant;
use ruxotics::{Payoff, Model, EuroOpt};

fn main() {
    let bs = Model {
        mu: 0.05,
        sigma: 0.17,
    };
    let c = Payoff {
        opt_type: EuroOpt::Call,
        maturity: 1.0f32,
        strike: 1.0f32,
    };
    let n: usize = 10000;

    let start = Instant::now();
    let res = c.price(&bs, n);
    let duration = start.elapsed();
    println!("single-threaded ...");
    println!("price of {:?} is {}", c, res);
    println!("pricing took {} milli-seconds", duration.as_millis());
}
