use std::fmt::Debug;
use crate::machine::dfa::HBRecord;

pub mod abstract_machine;
pub mod dfa;
pub mod state;

pub(crate) static PROB_GRAIN: u32 = 10000;
pub(crate) static DEFAULT_STATE_WAITING_TIME: u32 = 100;
pub(crate) static DEFAULT_WAIT_SPREAD: u32 = 10;

pub trait StateMachine {
    type StateValue: Debug;
    fn init(&mut self);
    fn change(&mut self);
    fn eval(&self) -> HBRecord;
    fn time(&self) -> u32;
    fn done(&self) -> bool;
    fn reset(&mut self);
    fn is_next_state_computed(&self) -> bool;
    fn set_is_next_state_computed(&mut self, flag:bool);
    fn set_current_time(&mut self, ct: u32);
}
