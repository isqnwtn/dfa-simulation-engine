use rlua::{Error, Lua};

use crate::machine::abstract_machine::AbstractMachine;
use crate::stream::globals::Globals;

use super::global_reader::GlobalReader;
use super::machine_reader::MachineReader;

pub struct Reader {
    lua: Lua,
    dsl: String,
}

impl Reader {
    pub fn new(dsl_str: &str) -> Self {
        Self {
            lua: Lua::new(),
            dsl: String::from(dsl_str),
        }
    }
    pub fn read_machine(&mut self) -> Result<AbstractMachine, Error> {
        self.lua.context(|lctx| {
            lctx.load(&self.dsl).set_name("dsl")?.exec()?;
            let mut mr = MachineReader::new(&lctx)?;
            mr.read_machine()
        })
    }
    pub fn read_globals(&mut self) -> Result<Globals, Error> {
        self.lua.context(|lctx| {
            lctx.load(&self.dsl).set_name("dsl")?.exec()?;
            let mut gr = GlobalReader::new(&lctx)?;
            gr.read_globals()
        })
    }
}
