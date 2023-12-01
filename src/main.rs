use std::env;

use crate::stream::multi_stream;

mod dsl;
mod machine;
mod stream;

fn main() -> Result<(), rlua::Error> {
    // parse arguments
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();
    let count = args
        .get(2)
        .unwrap_or(&String::from("20"))
        .parse::<u32>()
        .unwrap();
    let dfa_count = args
        .get(3)
        .unwrap_or(&String::from("4"))
        .parse::<usize>()
        .unwrap();

    // read machine from dsl
    let machine = dsl::read_machine(filename)?;
    println!("states: {machine:#?}");

    //running stream
    multi_stream(dfa_count, count, machine);
    Ok(())
}
