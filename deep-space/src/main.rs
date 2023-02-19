use bones::{add, run};

fn main() {
    println!("Hello, world! {}", add(1, 2));

    pollster::block_on(run());

}
