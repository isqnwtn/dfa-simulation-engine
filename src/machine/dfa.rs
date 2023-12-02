use super::PROB_GRAIN;
use super::{abstract_machine::AbstractMachine, StateMachine};
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use std::char;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
pub struct DFA<'a> {
    machine: &'a AbstractMachine,
    current_state: String,
    current_time: u32,
    done: bool,
    session_id: String,
    rng: ThreadRng,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct HBRecord {
    timestamp: u32,
    session_id: String,
    data: String
}

impl<'a> DFA<'a> {
    pub fn new(m: &'a AbstractMachine, start_time: u32) -> Self {
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
            current_time: start_time,
            done: false,
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
        if !self.done {
            let cur = self.machine.get_state(&self.current_state);
            self.current_time += cur.get_time_spent(&mut self.rng);
            let p = self.rng.gen_range(1..=PROB_GRAIN);
            if let Some(st) = cur.matching(p) {
                if self.machine.is_final_state(&st) {
                    self.done = true;
                }
                self.current_state = st;
            } else {
                self.done = true;
            }
        }
    }

    fn eval(&self) -> HBRecord {
        return HBRecord {
            timestamp: self.current_time,
            session_id: self.session_id.clone(),
            data: self.current_state.clone(),
        }
    }

    fn time(&self) -> u32 {
        self.current_time
    }

    fn done(&self) -> bool {
        self.done
    }

    fn reset(&mut self) {
        self.current_state = self.machine.get_default_state();
        self.done = false;
        self.current_time = 0;
    }
}

impl PartialEq for DFA<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.session_id == other.session_id
    }
}

impl Eq for DFA<'_> {}

impl Hash for DFA<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.session_id.hash(state)
    }
}
