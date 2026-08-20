#[path = "ch03-types.rs"]
mod ch03_types;
#[path = "ch04-ownership.rs"]
mod ch04_ownership;

fn main() {
    ch03_types::run();
    ch04_ownership::run();
}
