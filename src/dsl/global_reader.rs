use std::collections::HashMap;

use rlua::{Context, Error, Table};

use crate::machine::abstract_machine::AbstractMachine;
use crate::machine::state::State;
use crate::machine::{DEFAULT_STATE_WAITING_TIME, DEFAULT_WAIT_SPREAD};

pub struct GlobalReader<'a> {
    global_table: Table<'a>,
}
impl<'a> GlobalReader<'a> {
    pub fn new(lctx: &Context<'a>) -> Result<Self, Error> {
        let g = lctx.globals();
        let m: Vec<Table> = g.get("GLOBAL").expect("Couldn't read GLOBAL");
        Ok(GlobalReader { global_table: m })
    }
}
