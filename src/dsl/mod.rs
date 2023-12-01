pub mod reader;

use rlua;
use std::fs;

use crate::machine::abstract_machine::DFA;
use crate::stream::stream;

pub fn test_reader(filepath: &'static str) -> Result<(), rlua::Error> {
    //reading the dsl file
    let dsl_string = fs::read_to_string(filepath).expect("couldn't read dsl file");
    //println!("{}", dsl_string);

    //reader execution
    let mut reader = reader::Reader::new(&dsl_string);
    let machine = reader.read_machine()?;
    println!("states: {:#?}", machine);
    let mut dfa = DFA::new(machine);

    stream(10, &mut dfa);

    Ok(())
}
