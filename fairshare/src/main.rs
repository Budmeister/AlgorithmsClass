
mod error;
mod input;
mod fairshare;

use error::Result;
use input::Coins;

fn main() -> Result<()> {
    for coins in input::get_input()? {
        let coins: Coins = coins?;
        let diff = fairshare::solve_fs(&coins);
        println!("{}", diff);
    }
    Ok(())
}
