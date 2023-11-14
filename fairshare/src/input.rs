use std::{
    io,
    num,
    fmt,
};

pub type Result<T> = core::result::Result<T, Error>;
pub type Coins = Vec<usize>;
pub type RefCoins<'a> = &'a [usize];

pub fn get_input() -> Result<FSInputIterator> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let len = buf.trim().parse()?;
    Ok(FSInputIterator { len , cur: 0, buf })
}

pub struct FSInputIterator {
    len: usize,
    cur: usize,
    buf: String,
}
impl FSInputIterator {
    pub fn len(&mut self) -> usize {
        self.len
    }
}
impl Iterator for FSInputIterator {
    type Item = Result<Coins>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.len {
            self.cur += 1;
            Some(get_fs_input(&mut self.buf))
        } else {
            None
        }
    }
}



fn get_fs_input(buf: &mut String) -> Result<Coins> {
    buf.clear();
    io::stdin().read_line(buf)?;
    let _len: usize = buf.trim().parse()?;

    buf.clear();
    io::stdin().read_line(buf)?;
    let coins: Coins = buf.trim().split(' ')
        .map(|coin| Ok(coin.parse()?))
        .collect::<Result<Coins>>()?;
    Ok(coins)
}




#[derive(fmt::Debug)]
pub enum Error {
    IOErr(io::Error),
    ParseIntErr(num::ParseIntError),
}
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOErr(value)
    }
}
impl From<num::ParseIntError> for Error {
    fn from(value: num::ParseIntError) -> Self {
        Self::ParseIntErr(value)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOErr(err) => write!(f, "{}", err),
            Self::ParseIntErr(err) => write!(f, "{}", err),
        }
    }
}
