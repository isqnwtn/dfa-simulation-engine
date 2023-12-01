use super::PROB_GRAIN;
use super::{abstract_machine::AbstractMachine, StateMachine};
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use std::char;

#[derive(Clone)]
pub struct DFA<'a> {
    machine: &'a AbstractMachine,
    current_state: String,
    current_time: u32,
    session_id: String,
    rng: ThreadRng,
}

impl<'a> DFA<'a> {
    pub fn new(m: &'a AbstractMachine) -> Self {
        let default_state = m.get_default_state();
        let id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect::<String>();
        DFA {
            machine: m,
            session_id: id,
            current_state: default_state,
            current_time: 0,
            rng: rand::thread_rng(),
        }
    }
}

impl<'a> StateMachine for DFA<'a> {
    type StateValue = String;
    fn init(&mut self) {
        self.current_state = self.machine.get_default_state()
    }
    fn change(&mut self) {
        let cur = self.machine.get_state(&self.current_state);
        self.current_time += cur.get_time_spent(&mut self.rng);
        let p = self.rng.gen_range(1..=PROB_GRAIN);
        if let Some(st) = cur.matching(p) {
            self.current_state = st;
        }
    }
    fn eval(&self) -> Self::StateValue {
        format!(
            "timestamp:{} session:{} data:{}",
            self.current_time, self.session_id, self.current_state
        )
    }
    fn reset(&mut self) {
        self.current_state = self.machine.get_default_state()
    }
}
