use rlua::{Function, Lua};

pub struct Globals{
    pub max_sessions: usize,
    pub run_length: usize,
}

impl Globals{
    pub fn new(m:usize,r:usize) -> Self{
        Globals{
            max_sessions: m,
            run_length: r,
        }
    }
}

pub struct GlobalState {
    state_lua: Lua
}

impl GlobalState {
    pub fn new(l:Lua) -> Self {
        GlobalState { state_lua: l }
    }
}
