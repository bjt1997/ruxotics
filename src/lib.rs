use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;
use std::f32;

pub struct Model {
    pub mu: f32,
    pub sigma: f32,
}

impl Model {
    pub fn gen(&self, n: usize, dt: f32) -> f32 {
        let mut logx = 0.0f32;
        let mut rng = thread_rng();
        let r: f32 = (self.mu - 0.5 * self.sigma * self.sigma) * dt;
        let v: f32 = self.sigma * dt.sqrt();
        for _i in 0..n {
            let z: f32 = rng.sample(StandardNormal);
            logx += r + v * z;
        }
        logx.exp()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EuroOpt {
    Call,
    Put,
}

#[derive(Debug)]
pub struct Payoff {
    pub opt_type: EuroOpt,
    pub maturity: f32,
    pub strike: f32,
}

impl Payoff {
    pub fn payout(&self, x: f32) -> f32 {
        match self.opt_type {
            EuroOpt::Call => x.max(self.strike) - self.strike,
            EuroOpt::Put => self.strike - x.min(self.strike),
        }
    }
    pub fn price(&self, m: &Model, n: usize) -> f32 {
        let mut res = 0.0f32;
        for _i in 0..n {
            let p = m.gen(1, self.maturity);
            res += self.payout(p);
        }
        res /= n as f32;
        res
    }
}
