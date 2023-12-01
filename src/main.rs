use std::env;

mod dsl;
mod machine;
mod stream;

fn main() -> Result<(), rlua::Error> {
    let args : Vec<String> = env::args().collect();
    let filename = &args[1];
    let count = args[2].parse::<u32>().unwrap();
    dsl::test_reader(filename,count)
}
