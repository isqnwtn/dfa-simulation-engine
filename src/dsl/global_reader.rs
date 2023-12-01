use rlua::{Context, Error, Table};
use crate::stream::globals::{self, Globals};


pub struct GlobalReader<'a> {
    global_table: Table<'a>,
}
impl<'a> GlobalReader<'a> {
    pub fn new(lctx: &Context<'a>) -> Result<Self, Error> {
        let g = lctx.globals();
        let m: Table = g.get("GLOBAL").expect("Couldn't read GLOBAL");
        Ok(GlobalReader { global_table: m })
    }
    pub fn read_globals(&mut self) -> Result<globals::Globals, Error> {
        let max_sessions = self.global_table.get::<&str,usize>("max_sessions")?;
        let run_length = self.global_table.get::<&str,usize>("run_length")?;
        Ok(Globals::new(max_sessions, run_length))
    }
}
