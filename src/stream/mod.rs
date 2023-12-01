pub mod globals;

use crate::machine::abstract_machine::AbstractMachine;
use crate::machine::dfa::DFA;
use crate::machine::StateMachine;
use priority_queue::double_priority_queue;

use self::globals::Globals;

pub struct StreamEngine {
    globals: Globals,
    abstract_machine: AbstractMachine
}

impl StreamEngine {
    pub fn new(g: Globals, a: AbstractMachine) -> Self {
        StreamEngine { globals: g, abstract_machine: a }
    }

    pub fn multi_stream(&self) {
        // use priority queue to oreder by time
        let mut dfa_q: double_priority_queue::DoublePriorityQueue<DFA, u32> =
            double_priority_queue::DoublePriorityQueue::new();

        // insert all DFAs to the queue
        for _ in 0..self.globals.max_sessions {
            let dfa = DFA::new(&self.abstract_machine, 0);
            dfa_q.push(dfa, 0);
        }

        let mut last_recorded_time = 0;
        for _ in 0..(self.globals.run_length * self.globals.max_sessions) {
            // pop the dfa which has the nearest heartbeat
            // popping will remove the item, only push it back if the DFA is not done
            let pop = dfa_q.pop_min();
            if let Some((mut dfa, _)) = pop {
                // process
                println!("HB: {:?}", dfa.eval());
                if !dfa.done() {
                    // update the dfa if its not done to calculate the time of next heartbeat
                    dfa.change();
                    let new_time = dfa.time();
                    last_recorded_time = new_time;
                    dfa_q.push(dfa, new_time);
                } else {
                    // if it's done create a new dfa and add it to the queue
                    // this maintains the total active sessions at a const which is given by `self.globals.max_sessions`
                    // TODO: use global variables to decide how many should be in each state
                    let new_dfa = DFA::new(&self.abstract_machine, last_recorded_time);
                    dfa_q.push(new_dfa, last_recorded_time);
                }
            } else {
                // if queue is empty and we haven't reached desired amount of heartbeats, start
                // creating new sessions
                let dfa = DFA::new(&self.abstract_machine, last_recorded_time);
                dfa_q.push(dfa, last_recorded_time);
            }
        }
    }
}
