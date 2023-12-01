use std::collections::HashMap;

use rlua::{Context, Error, Lua, Table, Value};

use crate::machine::abstract_machine::{AbstractMachine, State};

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

pub struct MachineReader<'a> {
    machine: Vec<Table<'a>>,
}
impl<'a> MachineReader<'a> {
    pub fn new(lctx: &Context<'a>) -> Result<Self, Error> {
        let g = lctx.globals();
        let m: Vec<Table> = g.get("MACHINE").expect("Couldn't read MACHINE");
        Ok(MachineReader { machine: m })
    }

    pub fn get_states(&mut self) -> Result<Vec<String>, Error> {
        Ok(self
            .machine
            .iter()
            .map(|t| -> String {
                t.get::<&str, String>("name")
                    .expect("couldn't read name of state")
            })
            .collect())
    }

    pub fn read_machine(&mut self) -> Result<AbstractMachine, Error> {
        // machine state map
        let mut map: HashMap<String, State> = HashMap::new();
        let mut default: Option<String> = None;
        for i in self.machine.iter() {
            // get the name of the state
            let name: String = i
                .get::<&str, String>("name")
                .expect("couldn't read state name");
            // decide if it's the default state
            if let Ok(true) = i.contains_key::<&str>("default") {
                default = Some(name.clone())
            }

            // get probabilities
            let probs_table: Result<Table<'_>, Error> = i.get::<&str, Table<'_>>("probs");
            let mut probs_vector: Vec<(String, f64)> = Vec::new();
            let mut total_prob = 0.;
            if let Ok(probs) = probs_table {
                for pair in probs.pairs::<String, f64>() {
                    let (key, probability) = pair?;
                    probs_vector.push((key, probability));
                    total_prob += probability;
                }
            }
            probs_vector.iter_mut().for_each(|v| v.1 = v.1 / total_prob);
            map.insert(name.clone(), State::new(&name, &probs_vector));
        }
        Ok(AbstractMachine::new(
            "Machine",
            &(default.expect("no default state found")),
            map,
        ))
    }
}
