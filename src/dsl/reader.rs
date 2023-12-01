use rlua::{Error, Lua};

use crate::machine::abstract_machine::AbstractMachine;

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
    pub fn get_states(&mut self) -> Result<Vec<String>, Error> {
        self.lua.context(|lctx| {
            lctx.load(&self.dsl).set_name("dsl")?.exec()?;
            let mut mr = MachineReader::new(&lctx)?;
            mr.get_states()
        })
    }
    pub fn read_machine(&mut self) -> Result<AbstractMachine, Error> {
        self.lua.context(|lctx| {
            lctx.load(&self.dsl).set_name("dsl")?.exec()?;
            let mut mr = MachineReader::new(&lctx)?;
            mr.read_machine()
        })
    }
}
