
#[macro_use]
extern crate macro_sandbox_lib;

#[route(GET, "/")]
fn first_function() {
    println!("first_function")
}

#[show_streams]
fn second_function() {
    println!("second_function")
}

fn main() {
    println!("Hello, world!");
    first_function();
    second_function();
}
