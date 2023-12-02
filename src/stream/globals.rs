use rlua::{Function, Lua, Error, Table};

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
    pub fn run_session_manager(&mut self,cur_time:u32) -> Result<usize,Error> {
        self.state_lua.context(|lctx|{
            let globals = lctx.globals();
            let session_manager: Function = globals.get::<&str,Function<'_>>("SESSION_MANAGER")?;
            let new_session_count = session_manager.call::<i32,i32>(cur_time as i32)?;
            Ok(new_session_count as usize)
        })
    }
}
