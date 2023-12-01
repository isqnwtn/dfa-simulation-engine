use std::collections::HashMap;

use rlua::{Context, Error, Table};

use crate::machine::abstract_machine::AbstractMachine;
use crate::machine::state::State;
use crate::machine::{DEFAULT_STATE_WAITING_TIME, DEFAULT_WAIT_SPREAD};

pub struct MachineReader<'a> {
    machine: Vec<Table<'a>>,
}
impl<'a> MachineReader<'a> {
    pub fn new(lctx: &Context<'a>) -> Result<Self, Error> {
        let g = lctx.globals();
        let m: Vec<Table> = g.get("MACHINE").expect("Couldn't read MACHINE");
        Ok(MachineReader { machine: m })
    }

    pub fn read_machine(&mut self) -> Result<AbstractMachine, Error> {
        // machine state map
        let mut map: HashMap<String, State> = HashMap::new();
        let mut default: Option<String> = None;
        let mut final_state: Option<String> = None;
        for i in self.machine.iter() {
            // optional variables
            let mut wait_time: Option<u32> = None;
            let mut wait_spread: Option<u32> = None;

            // get the name of the state
            let name: String = i
                .get::<&str, String>("name")
                .expect("couldn't read state name");

            // decide if it's the default state
            if let Ok(true) = i.contains_key::<&str>("default") {
                default = Some(name.clone())
            }

            // decide if it's the default state
            if let Ok(true) = i.contains_key::<&str>("final") {
                final_state = Some(name.clone())
            }

            //extract wait time if exists
            if let Ok(true) = i.contains_key::<&str>("waitTime") {
                wait_time = Some(
                    i.get::<&str, u32>("waitTime")
                        .expect("couldn't read wait time although field exists"),
                );
            }

            //extract wait spread if exists
            if let Ok(true) = i.contains_key::<&str>("waitSpread") {
                wait_spread = Some(
                    i.get::<&str, u32>("waitSpread")
                        .expect("couldn't read wait spread although field exists"),
                );
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
            probs_vector.iter_mut().for_each(|v| v.1 /= total_prob);
            map.insert(
                name.clone(),
                State::new(
                    &name,
                    &probs_vector,
                    wait_time.unwrap_or(DEFAULT_STATE_WAITING_TIME),
                    wait_spread.unwrap_or(DEFAULT_WAIT_SPREAD),
                ),
            );
        }
        Ok(AbstractMachine::new(
            "Machine",
            &(default.expect("no default state found")),
            &(final_state.expect("no final state found")),
            map,
        ))
    }
}
