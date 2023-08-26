use crate::world::World;

pub fn print_world(world: &World) {
    for (i, tower) in world.iter().enumerate() {
        print!("{}:", i);
        for block in tower {
            print!(" {}", block);
        }
        println!();
    }
}
