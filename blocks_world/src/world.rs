use crate::instr::{FromTo, Instr};
use std::ops::Deref;

pub type Block = usize;
pub type Geography = [Vec<Block>];
#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub c: usize,
    pub h: usize,
}
pub type Index = [Location];  // column, height
pub struct World {
    geo: Box<Geography>,
    ind: Box<Index>,
}
impl World {
    /// Put block `a` onto block `b` after returning any blocks stacked on top
    /// of `a` and `b` (clearing them) to their original locations.
    pub fn move_onto(&mut self, FromTo { a, b }: FromTo) {
        self.clear(a);
        self.clear(b);
        self.move_top_of_column(
            self.get_location(a).c,
            self.get_location(b).c,
        );
    }
    /// Put block `a` onto the top of the stack containing block `b` after
    /// returning any blocks stacked on top of block `a` (clearing them)
    /// to their original locations.
    pub fn move_over(&mut self, FromTo { a, b }: FromTo) {
        self.clear(a);
        self.move_top_of_column(
            self.get_location(a).c,
            self.get_location(b).c,
        );
    }
    /// Move the pile consisting of block `a` and any blocks stacked on top
    /// of it to be on top of block `b` after clearing block `b`.
    pub fn pile_onto(&mut self, FromTo { a, b }: FromTo) {
        self.clear(b);
        self.move_block_and_above(
            self.get_location(a),
            self.get_location(b).c,
        );
    }
    /// Move the pile consisting of block `a` and any blocks stacked on top
    /// of it to the top of the stack containing block `b`.
    pub fn pile_over(&mut self, FromTo { a, b }: FromTo) {
        self.move_block_and_above(
            self.get_location(a),
            self.get_location(b).c,
        );
    }

    pub fn is_instr_illegal(&self, instr: &Instr) -> bool {
        let FromTo { a, b } = match instr {
            Instr::MoveOnto(ft) => *ft,
            Instr::MoveOver(ft) => *ft,
            Instr::PileOnto(ft) => *ft,
            Instr::PileOver(ft) => *ft,
            Instr::Quit => return false,
        };
        if a == b {
            return true;
        }
        if self.get_location(a).c == self.get_location(b).c {
            return true;
        }
        false
    }

    fn get_location(&self, block: Block) -> Location {
        self.ind[block]
    }
    fn get_tower_mut(&mut self, c: usize) -> &mut Vec<Block> {
        &mut self.geo[c]
    }
    /// Returns all the blocks on top of `block`.
    /// 
    /// Preserves reduncancy consistency.
    fn clear(&mut self, block: Block) {
        let Location { c, h } = self.get_location(block);
        let tower_height = self.geo[c].len();
        let num_to_return = tower_height - h - 1;
        for _ in 0..num_to_return {
            self.return_(c);
        }
    }
    /// Returns the block on the top of the given tower to its original location.
    /// 
    /// Panics if its original location is already populated or if the given tower 
    /// is empty.
    /// 
    /// Preserves reduncancy consistency.
    fn return_(&mut self, c: usize) {
        let tower = self.get_tower_mut(c);
        let block: Block = *tower.last().unwrap();
        if !self.geo[block].is_empty() {
            panic!("Trying to return block {}, but it's tower is not empty: {:?}", block, self.geo[block]);
        }
        self.move_top_of_column(c, block);
    }
    /// Moves the block from the top of tower, `from_c` to the top of the tower, `to_c`.
    /// 
    /// Panics if tower, `from_c` is empty.
    /// 
    /// Preserves reduncancy consistency.
    fn move_top_of_column(&mut self, from_c: usize, to_c: usize) {
        let block: Block = self.geo[from_c].pop().unwrap();
        self.geo[to_c].push(block);
        let new_height = self.geo[to_c].len();
        self.ind[block] = Location { c: to_c, h: new_height - 1 };
    }
    fn move_block_and_above(&mut self, Location { c: from_c, h }: Location, to_c: usize) {
        let (to_leave, to_move) = self.geo[from_c].split_at(h);
        let to_leave: Vec<_> = to_leave.into_iter().map(|x| *x).collect();
        let to_move: Vec<_> = to_move.into_iter().map(|x| *x).collect();
        self.geo[from_c] = to_leave;
        for block in to_move {
            self.ind[block] = Location { c: to_c, h: self.geo[to_c].len() };
            self.geo[to_c].push(block);
        }
    }
}
impl Deref for World {
    type Target = Geography;

    fn deref(&self) -> &Self::Target {
        &self.geo
    }
}

pub fn create_world(num_blocks: usize) -> World {
    let ind: Box<Index> = (0..num_blocks)
        .map(|x| Location { c: x, h: 0 })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let geo: Box<Geography> = (0..num_blocks)
        .map(|x| vec![x])
        .collect::<Vec<_>>()
        .into_boxed_slice();
    World { ind, geo }
}
