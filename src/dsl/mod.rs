pub mod reader;

use rlua;
use std::fs;

use crate::machine::abstract_machine::AbstractMachine;

pub fn read_machine<'a>(filepath: &'a str) -> Result<AbstractMachine, rlua::Error> {
    //reading the dsl file
    let dsl_string = fs::read_to_string(filepath).expect("couldn't read dsl file");

    //reader execution
    let mut reader = reader::Reader::new(&dsl_string);
    let machine = reader.read_machine()?;
    Ok(machine)
}
