use crate::{
    instr::Instr,
    world::World,
    error::DEBUG,
};


pub fn perform_instrs(instrs: &[Instr], world: &mut World) {
    for instr in instrs {
        if !perform_instr(instr, world) {
            break;
        }
    }
}

/// Returns true if processing should continue
fn perform_instr(instr: &Instr, world: &mut World) -> bool {
    if DEBUG {
        println!("Performing Instr {:?}", instr);
    }
    if world.is_instr_illegal(instr) {
        if DEBUG {
            println!("Ignoring");
        }
        return true;
    }
    use Instr::*;
    match instr {
        MoveOnto(ft) => world.move_onto(*ft),
        MoveOver(ft) => world.move_over(*ft),
        PileOnto(ft) => world.pile_onto(*ft),
        PileOver(ft) => world.pile_over(*ft),
        Quit => return false,
    }
    true
}

