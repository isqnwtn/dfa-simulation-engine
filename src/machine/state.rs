use crate::machine::PROB_GRAIN;
use rand::prelude::*;



#[derive(Debug, Clone)]
pub struct TransferInfo {
    name: String,
    probability: f64,
    start: u32,
    end: u32,
}

#[derive(Debug, Clone)]
pub struct State {
    name: String,
    wait_time: u32,
    wait_spread: u32,
    probs: Vec<TransferInfo>,
}

impl State {
    pub fn new(n: &str, v: &[(String, f64)], w: u32, w_spread: u32) -> Self {
        let mut stamp = 0;
        let transfer_vec = v
            .iter()
            .map(|v| {
                let s = stamp;
                let e = s + ((v.1 * (PROB_GRAIN as f64)) as u32);
                stamp = e + 1;
                TransferInfo {
                    name: v.0.clone(),
                    probability: v.1,
                    start: s,
                    end: if e > PROB_GRAIN { PROB_GRAIN } else { e },
                }
            })
            .collect();
        State {
            name: n.to_string(),
            wait_time: w,
            wait_spread: w_spread,
            probs: transfer_vec,
        }
    }
    pub fn matching(&self, p: u32) -> Option<String> {
        for i in self.probs.iter() {
            if p >= i.start && p <= i.end {
                return Some(i.name.clone());
            }
        }
        None
    }
    #[inline]
    pub fn get_time_spent(&self, rng: &mut ThreadRng) -> u32 {
        self.wait_time + rng.gen_range(0..self.wait_spread)
    }
}
