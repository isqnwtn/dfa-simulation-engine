mod dsl;
mod machine;
mod stream;

fn main() {
    //stream::stream(30);
    dsl::test_reader("dsl/test.lua");
}
