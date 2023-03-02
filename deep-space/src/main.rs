use bones::Component;

fn main() {
    println!("Hello, world! {}", bones::add(1, 2));

    #[derive(Component)]
    struct Player;

    // bones::Skeleton::new()
    //      .insert_resources()
    //      .insert_entities()
    //      .run()

    pollster::block_on(bones::run());
}
