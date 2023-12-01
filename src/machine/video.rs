use super::StateMachine;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub enum PlayerState {
    Inactive,
    Playing,
    Paused,
    Buffering,
    Seeking,
    Stopped,
}

pub struct Player {
    state: PlayerState,
    rng: ThreadRng,
}

impl Player {
    pub fn new() -> Self {
        Player {
            state: PlayerState::Inactive,
            rng: rand::thread_rng(),
        }
    }
}

impl StateMachine for Player {
    type StateValue = PlayerState;
    fn init(&mut self) {
        self.state = PlayerState::Inactive;
        self.rng = rand::thread_rng();
    }
    fn eval(&self) -> Self::StateValue {
        self.state.clone()
    }
    fn reset(&mut self) {
        self.state = PlayerState::Inactive;
    }
    fn change(&mut self) {
        match &self.state {
            PlayerState::Inactive => {
                let p = self.rng.gen_range(1..=100);
                if p < 98 {
                    self.state = PlayerState::Playing;
                } else {
                    self.state = PlayerState::Stopped;
                }
            }
            PlayerState::Playing => {
                let p = self.rng.gen_range(0..=100);
                if p < 40 {
                    self.state = PlayerState::Playing
                } else if p < 70 {
                    self.state = PlayerState::Buffering
                } else if p < 90 {
                    self.state = PlayerState::Paused
                } else {
                    self.state = PlayerState::Seeking
                }
            }
            PlayerState::Buffering => {
                let p = self.rng.gen_range(0..=100);
                if p < 60 {
                    self.state = PlayerState::Playing
                } else if p < 70 {
                    self.state = PlayerState::Paused
                } else if p < 80 {
                    self.state = PlayerState::Seeking
                } else if p < 90 {
                    self.state = PlayerState::Stopped
                }
            }
            PlayerState::Seeking => {
                let p = self.rng.gen_range(0..=100);
                if p < 70 {
                    self.state = PlayerState::Buffering
                } else if p < 90 {
                    self.state = PlayerState::Playing
                } else {
                    self.state = PlayerState::Seeking
                }
            }
            PlayerState::Paused => {
                let p = self.rng.gen_range(0..=100);
                if p < 50 {
                    self.state = PlayerState::Playing
                } else {
                    self.state = PlayerState::Stopped
                }
            }
            _ => self.state = PlayerState::Inactive,
        }
    }
}
