use std::fmt::Debug;

pub mod abstract_machine;
pub mod dfa;

pub(crate) static PROB_GRAIN: u32 = 10000;
pub(crate) static DEFAULT_STATE_WAITING_TIME: u32 = 100;


pub trait StateMachine {
    type StateValue: Debug;
    fn init(&mut self);
    fn change(&mut self);
    fn eval(&self) -> Self::StateValue;
    fn reset(&mut self);
}
