use crate::instr::FromTo;


pub type Block = usize;
pub type Geography = [Vec<Block>];
pub type Index = [usize];
pub struct World {
    geo: Box<Geography>,
    ind: Box<Index>,
}
impl World {
    pub fn move_onto(&mut self, FromTo { a, b }: FromTo) {
        todo!()
    }
    pub fn move_over(&mut self, FromTo { a, b }: FromTo) {
        todo!()
    }
    pub fn pile_onto(&mut self, FromTo { a, b }: FromTo) {
        todo!()
    }
    pub fn pile_over(&mut self, FromTo { a, b }: FromTo) {
        todo!()
    }
}

pub fn create_world(num_blocks: usize) -> World {
    todo!()
}
