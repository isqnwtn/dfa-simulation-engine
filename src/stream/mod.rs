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
    //let mut rng = rand::thread_rng();
    let mut dfa_q: double_priority_queue::DoublePriorityQueue<DFA, u32> =
        double_priority_queue::DoublePriorityQueue::new();

    for _ in 0..dfa_count {
        let dfa = DFA::new(&ab_state_machine,0);
        dfa_q.push(dfa,0);
    }
    let mut last_recorded_time = 0;
    for _ in 0..(count * dfa_count as u32) {
        if !dfa_q.is_empty() {
            let pop = dfa_q.pop_min();
            if let Some((mut dfa, _)) = pop {
                println!("HB: {:?}", dfa.eval());
                if !dfa.done(){
                    dfa.change();
                    let new_time = dfa.time();
                    last_recorded_time = new_time;
                    dfa_q.push(dfa, new_time);
                } else {
                    let new_dfa = DFA::new(&ab_state_machine,last_recorded_time);
                    dfa_q.push(new_dfa, last_recorded_time);
                }
                
            } else {
                let dfa = DFA::new(&ab_state_machine,last_recorded_time);
                dfa_q.push(dfa, last_recorded_time);
            }
        }
    }
}
