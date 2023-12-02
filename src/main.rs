use std::env;

use crate::dsl::DSL;
use crate::stream::globals;

use self::stream::StreamEngine;

mod dsl;
mod machine;
mod stream;
mod io;

fn main() -> Result<(), rlua::Error> {
    // parse arguments
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();

    // read machine from dsl
    let dsl = DSL::new(filename);
    let machine = dsl.read_machine()?;
    let globals = dsl.read_globals()?;
    let stream_engine = StreamEngine::new(globals,machine);
    stream_engine.multi_stream();

    //running stream
    Ok(())
}
