use rlua::{Error, Lua};

use crate::machine::abstract_machine::AbstractMachine;
use crate::stream::globals::{Globals, GlobalState};

use super::global_reader::GlobalReader;
use super::machine_reader::MachineReader;

pub struct Reader {
    lua: Lua,
    dsl: String,
}

impl Reader {
    pub fn new(dsl_str: &str) -> Result<Self,Error> {
        let l = Lua::new();
        l.context(|lctx|{
            lctx.load(dsl_str).set_name("dsl")?.exec()
        })?;
        Ok(Self {
            lua: l,
            dsl: String::from(dsl_str),
        })
    }
    pub fn read_machine(&mut self) -> Result<AbstractMachine, Error> {
        self.lua.context(|lctx| {
            let mut mr = MachineReader::new(&lctx)?;
            mr.read_machine()
        })
    }
    pub fn read_globals(&mut self) -> Result<Globals, Error> {
        self.lua.context(|lctx| {
            let mut gr = GlobalReader::new(&lctx)?;
            gr.read_globals()
        })
    }
    pub fn create_global_state(&self) -> Result<GlobalState,Error> {
        let l = Lua::new();
        l.context(|lctx|{
            lctx.load(&self.dsl).set_name("global_state")?.exec()
        })?;
        Ok(GlobalState::new(l))
    }
}
