use crate::machine::abstract_machine::AbstractMachine;
use crate::machine::dfa::DFA;
use crate::machine::StateMachine;
use priority_queue::double_priority_queue;

#[allow(unused_parens)]

pub fn stream(count: u32, dfa: &mut impl StateMachine) {
    dfa.init();
    for _i in (0..count) {
        dfa.change();
        println!("HB: {:?}", dfa.eval())
    }
}

pub fn multi_stream(dfa_count: usize, count: u32, ab_state_machine: AbstractMachine) {
    // use priority queue to oreder by time
    let mut dfa_q: double_priority_queue::DoublePriorityQueue<DFA, u32> =
        double_priority_queue::DoublePriorityQueue::new();

    // insert all DFAs to the queue
    for _ in 0..dfa_count {
        let dfa = DFA::new(&ab_state_machine, 0);
        dfa_q.push(dfa, 0);
    }

    let mut last_recorded_time = 0;
    for _ in 0..(count * dfa_count as u32) {
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
                // this maintains the total active sessions at a const which is given by `dfa_count`
                // TODO: use global variables to decide how many should be in each state
                let new_dfa = DFA::new(&ab_state_machine, last_recorded_time);
                dfa_q.push(new_dfa, last_recorded_time);
            }
        } else {
            // if queue is empty and we haven't reached desired amount of heartbeats, start
            // creating new sessions
            let dfa = DFA::new(&ab_state_machine, last_recorded_time);
            dfa_q.push(dfa, last_recorded_time);
        }
    }
}
