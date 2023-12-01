use std::fmt::Debug;

pub mod abstract_machine;
pub mod video;

pub trait StateMachine {
    type StateValue: Debug;
    fn init(&mut self);
    fn change(&mut self);
    fn eval(&self) -> Self::StateValue;
    fn reset(&mut self);
}
