use std::env;

use crate::machine::dfa::DFA;
use crate::stream::stream;

mod dsl;
mod machine;
mod stream;

fn main() -> Result<(), rlua::Error> {

    // parse arguments
    let args : Vec<String> = env::args().collect();
    let filename = &args[1];
    let count = args[2].parse::<u32>().unwrap();


    // read machine from dsl
    let machine = dsl::read_machine(filename)?;
    println!("states: {:#?}", machine);
    let mut dfa = DFA::new(&machine);

    //running stream
    stream(count, &mut dfa);
    Ok(())
}
