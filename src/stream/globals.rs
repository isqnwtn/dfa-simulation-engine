use rlua::Function;

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

pub struct GlobalState<'a> {
    session_manager: Function<'a>,
}
