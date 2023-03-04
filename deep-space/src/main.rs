use bones::prelude::*;

#[derive(Component)]
struct Player {
    health: u32
}

fn main() {

    Skeleton::new()
        .add_system(print_hello)
        .run()

}

fn print_hello(world: &mut World) {
    println!("Hello!");
}
