use bones::prelude::{*, light::PointLight};

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

    let cube = world.spawn_entity();
    let mut model = world.load_model("cube.obj").unwrap();
    
    // Calculate instances for cube
    const NUM_INSTANCES_PER_ROW: u32 = 10;
    const SPACE_BETWEEN: f32 = 3.0;
    model.instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
        (0..NUM_INSTANCES_PER_ROW).map(move |x| {
            let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
            let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

            let position = Vector3 { x, y: 0.0, z };

            let rotation = if position.is_zero() {
                Quaternion::from_axis_angle(Vector3::unit_z(), Deg(0.0))
            } else {
                Quaternion::from_axis_angle(position.normalize(), Deg(45.0))
            };

            Instance {
                position, rotation,
            }
        })
    }).collect::<Vec<_>>();

    world.add_component_to_entity(cube, model);

    let light = world.spawn_entity();
    let mut point_light = PointLight::new();
    point_light.uniform.color = [1.0, 0.0, 0.0];
    world.add_component_to_entity(light, point_light);
}
