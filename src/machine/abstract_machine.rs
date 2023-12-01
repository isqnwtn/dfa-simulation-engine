use std::collections;

use super::state::State;

#[derive(Debug)]
pub struct AbstractMachine {
    default: String,
    final_state: String,
    states: collections::HashMap<String, State>,
}

impl AbstractMachine {
    // TODO: remove unused _n
    pub fn new(_n: &str, d: &str, f: &str, s: collections::HashMap<String, State>) -> Self {
        AbstractMachine {
            default: d.to_string(),
            final_state: f.to_string(),
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
    pub fn is_final_state(&self, s: &str) -> bool {
        self.final_state.as_str() == s
    }
}
