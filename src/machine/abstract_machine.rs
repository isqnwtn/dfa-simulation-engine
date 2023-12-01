use std::collections;

use super::PROB_GRAIN;

#[derive(Debug)]
pub struct AbstractMachine {
    name: String,
    default: String,
    states: collections::HashMap<String, State>,
}

impl AbstractMachine {
    pub fn new(n: &str, d: &str, s: collections::HashMap<String, State>) -> Self {
        AbstractMachine {
            name: n.to_string(),
            default: d.to_string(),
            states: s,
        }
    }
    pub fn get_default_state(&self) -> String {
        self.default.clone()
    }
    pub fn get_state(&self, name: &str) -> State {
        let st = self.states.get(name).expect("unexpected state name");
        st.clone()
    }
}

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
    probs: Vec<TransferInfo>,
}

impl State {
    pub fn new(n: &str, v: &[(String, f64)]) -> Self {
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
            probs: transfer_vec,
        }
    }
    pub fn matching(&self,p:u32) -> Option<String> {
        for i in self.probs.iter(){
            if p >= i.start && p <= i.end {
                return Some( i.name.clone() );
            }
        }
        None
    }
}

