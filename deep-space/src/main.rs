use bones::prelude::*;

struct Player {
    health: u32
}

fn main() {

    Skeleton::new()
        .add_init_system(init_world)
        .run()

}

fn init_world(world: &mut World) {
    let entity = world.spawn_entity();
    world.add_component_to_entity(entity, 
        Player {
            health: 100
    });
}
