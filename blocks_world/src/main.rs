
mod instr;
mod world;
mod perform;
mod output;
mod error;

fn main() {
    let (instrs, num_blocks) = match instr::read_instrs() {
        Ok(x) => x,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    let mut world = world::create_world(num_blocks);
    if error::DEBUG {
        instr::print_instrs(&instrs, num_blocks);
        println!("World: ");
        output::print_world(&world);
    }

    perform::perform_instrs(&instrs, &mut world);
    output::print_world(&world);
}
