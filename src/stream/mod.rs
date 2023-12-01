

use crate::machine::abstract_machine::AbstractMachine;
use crate::machine::dfa::DFA;
use crate::machine::StateMachine;

#[allow(unused_parens)]

pub fn stream(count: u32, dfa: &mut impl StateMachine) {
    dfa.init();
    for i in (0..count) {
        dfa.change();
        println!("{} -> HB: {:?}", i, dfa.eval())
    }
}

pub fn multi_stream(dfa_count: usize, count: u32, ab_state_machine: AbstractMachine) {
    //let mut rng = rand::thread_rng();
    let mut dfa_vec: Vec<DFA> = Vec::new();
    for _ in 0..dfa_count {
        let dfa = DFA::new(&ab_state_machine);
        dfa_vec.push(dfa);
    }
    for _ in 0..(count * dfa_count as u32) {
        for j in dfa_vec.iter_mut() {
            j.change();
            println!("HB: {:?}", j.eval())
        }
    }
}
