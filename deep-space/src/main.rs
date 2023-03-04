use bones::prelude::*;

fn main() {

    #[derive(Component)]
    struct Player {
        health: u32
    }

    // bones::Skeleton::new()
    //      .insert_resources()
    //      .insert_entities()
    //      .run()

    pollster::block_on(
        Skeleton::new()
            .run()
    );
}
