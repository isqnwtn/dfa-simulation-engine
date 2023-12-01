pub mod machine_reader;
pub mod global_reader;
pub mod reader;

use std::fs;

use crate::machine::abstract_machine::AbstractMachine;
use crate::stream::globals::{Globals, self};

pub struct DSL {
    dsl: String,
}

impl DSL {
    pub fn new(file: &str) -> Self{
        let dsl_string = fs::read_to_string(file).expect("couldn't read dsl file");
        DSL{
            dsl:dsl_string
        }
    }
    pub fn read_machine(&self) -> Result<AbstractMachine, rlua::Error>{
        let mut reader = reader::Reader::new(&self.dsl);
        let machine = reader.read_machine()?;
        Ok(machine)
    }
    pub fn read_globals(&self) -> Result<Globals, rlua::Error> {
        let mut reader = reader::Reader::new(&self.dsl);
        let globals = reader.read_globals()?;
        Ok(globals)
    }
}

