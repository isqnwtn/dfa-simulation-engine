use std::collections;

use super::state::State;

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
