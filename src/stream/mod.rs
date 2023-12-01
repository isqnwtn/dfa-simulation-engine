use crate::machine::StateMachine;

#[allow(unused_parens)]

pub fn stream(count: usize, dfa: &mut impl StateMachine) {
    dfa.init();
    for i in (0..count) {
        dfa.change();
        println!("{} -> HB: {:?}", i, dfa.eval())
    }
}
