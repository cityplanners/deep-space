fn main() {
    println!("Hello, world! {}", bones::add(1, 2));

    pollster::block_on(bones::run());

}
