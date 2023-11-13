use std::ops::{Index, IndexMut};


#[derive(Clone)]
pub struct TMatrix<T> {
    vec: Vec<T>,
}
impl<T> TMatrix<T> {
    pub fn new() -> TMatrix<T> {
        TMatrix { vec: Vec::new() }
    }
    pub fn from_flattened(vec: Vec<T>) -> TMatrix<T> {
        TMatrix { vec }
    }
    pub fn flattened(&self) -> &[T] {
        &self.vec
    }
    pub fn into_flattened(self) -> Vec<T> {
        self.vec
    }
    pub fn add_row(&mut self, row: Box<[T]>) {
        if row.len() < self.width() + 1 {
            panic!("Tried to add a row of length {} but needed one of at least {}", row.len(), self.width() + 1);
        }
        self.vec.append(&mut Vec::from(row));
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn height(&self) -> usize {
        todo!()
    }
    pub fn width(&self) -> usize {
        self.height()
    }
}
impl<T: Default> TMatrix<T> {
    pub fn add_row_default(&mut self) {
        let mut row = Vec::new();
        row.resize_with(self.width() + 1, || T::default());
        self.add_row(row.into());
    }
}
impl<T> Index<usize> for TMatrix<T> {
    type Output = [T];

    fn index(&self, mut index: usize) -> &Self::Output {
        let len = index + 1;
        index = (index + 1) * (index + 1) - 1;
        return &self.vec[index..index + len];
    }
}
impl<T> IndexMut<usize> for TMatrix<T> {
    fn index_mut(&mut self, mut index: usize) -> &mut Self::Output {
        let len = index + 1;
        index = (index + 1) * (index + 1) - 1;
        return &mut self.vec[index..index + len];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
