
use std::{
    io,
    io::prelude::*,
    str::FromStr,
    fmt::Debug
};

use crate::{
    error::{
        BWError,
        ParseInstrErr,
    },
    world::Block
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct FromTo {
    pub a: Block,
    pub b: Block,
}
impl From<(Block, Block)> for FromTo {
    fn from((a, b): (Block, Block)) -> Self {
        Self { a, b }
    }
}

enum MoveOrPile {
    Move,
    Pile,
}
impl FromStr for MoveOrPile {
    type Err = ParseInstrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "move" => Ok(Self::Move),
            "pile" => Ok(Self::Pile),
            _ => Err(format!("Invalid mop: {}", s).into())
        }
    }
}
enum OntoOrOver {
    Onto,
    Over,
}
impl FromStr for OntoOrOver {
    type Err = ParseInstrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "onto" => Ok(Self::Onto),
            "over" => Ok(Self::Over),
            _ => Err(format!("Invalid ooo: {}", s).into())
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Instr {
    MoveOnto(FromTo),
    MoveOver(FromTo),
    PileOnto(FromTo),
    PileOver(FromTo),
    Quit,
}
impl From<(MoveOrPile, usize, OntoOrOver, usize)> for Instr {
    fn from((mop, a, ooo, b): (MoveOrPile, usize, OntoOrOver, usize)) -> Self {
        use MoveOrPile::*;
        use OntoOrOver::*;
        use Instr::*;
        match (mop, ooo) {
            (Move, Onto) => MoveOnto((a, b).into()),
            (Move, Over) => MoveOver((a, b).into()),
            (Pile, Onto) => PileOnto((a, b).into()),
            (Pile, Over) => PileOver((a, b).into()),
        }
    }
}
impl FromStr for Instr {
    type Err = ParseInstrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let words: Vec<_> = s.split(" ").collect();
        match &words[..] {
            ["quit"] => {
                Ok(Instr::Quit)
            }
            [mop, a, ooo, b] => {
                let mop: MoveOrPile = mop.parse()?;
                let ooo: OntoOrOver = ooo.parse()?;
                let a: usize = a.parse()?;
                let b: usize = b.parse()?;
                Ok((mop, a, ooo, b).into())
            },
            _ => Err(format!("Invalid format for instruction: {}", s).into()),
        }
    }
}

pub type Instrs = Vec<Instr>;

pub fn read_instrs() -> Result<(Instrs, usize), BWError> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    let buffer = buffer.trim();
    let num_blocks: usize = buffer.parse()?;
    let mut instrs: Instrs = Instrs::new();

    for line in stdin.lock().lines() {
        let line = line?;
        let instr: Instr = line.parse()?;
        let should_break = instr == Instr::Quit;
        instrs.push(instr);
        if should_break {
            break;
        }
    }

    Ok((instrs, num_blocks))
}

pub fn print_instrs(instrs: &Instrs, num_blocks: usize) {
    println!("Number of blocks: {}", num_blocks);
    println!("Instrs:");
    for instr in instrs {
        println!("\t{:?}", instr);
    }
}
