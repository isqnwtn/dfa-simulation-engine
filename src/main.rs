use std::env;

use crate::dsl::DSL;
use self::stream::StreamEngine;

mod dsl;
mod machine;
mod stream;

fn main() -> Result<(), rlua::Error> {
    // parse arguments
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();

    // read machine from dsl
    let dsl = DSL::new(filename);
    let machine = dsl.read_machine()?;
    let globals = dsl.read_globals()?;
    let global_state = dsl.create_global_state()?;
    let stream_engine = StreamEngine::new(globals,global_state,machine);
    stream_engine.multi_stream();

    //running stream
    Ok(())
}
